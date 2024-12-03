use crate::{
    prelude::*,
    utils::get_redis_conn,
    types::{
        self,
        mongodb::User,
        UserSession,
        Role,
        SSS_PUB_TK,
        SSS_DATA_TK,
        EMAIL_TK,
        ROLESEL_PUB_TK,
        ROLESEL_DATA_TK
    },
};
use anyhow::Result;
use argon2::password_hash::rand_core::{ OsRng, RngCore };
use core::convert::TryFrom;
use hex;
use serde_json::json;

/// Store the session key prefix as a const so it can't be typo'd anywhere it's used.
const SESSION_KEY_PREFIX: &str = "session_";
/// Store the email key prefix as a const so it can't be typo'd anywhere it's used.
const EMAIL_KEY_PREFIX: &str = "email_";
/// Store the role selection key prefix as a const so it can't be typo'd anywhere it's used.
const ROLESELECT_KEY_PREFIX: &str = "roleselect_";

/// Issues a PASETO token to a user for storing the session.
/// Returns the session UUID token which should be set as a cookie,
/// and sets a key-value pair in Redis where this UUID is the key
/// and the session token is the value. This token has the user's id encoded.
#[tracing::instrument(name = "Issue PASETO token for session uuid", skip(redis_pool, user))]
pub async fn issue_session_token(
    user: User,
    role: Role,
    remember_me: bool,
    redis_pool: &deadpool_redis::Pool,
) -> Result<String> {
    let settings = crate::settings::get_settings().expect("Cannot read settings.");

    let sss_uuid = Uuid::new_v4();

    let mut redis_conn = if let Ok(conn) = get_redis_conn(redis_pool).await {
        conn
    } else {
        bail!(types::error::Redis::ConnError("Failed to obtain redis connection.".into()));
    };

    let redis_key = format!("{}{}", SESSION_KEY_PREFIX, sss_uuid);

    let user_id = user.id.clone();

    // Build the redis token containing the user id.
    let redis_token = build_sss_data_token(&settings, user, role).await?;

    redis_conn.set_ex::<_, _, ()>(redis_key, redis_token, settings.secret.session_token_expiration * 60).await?;

    // Build the session token to be set as a cookie, containing the UUID for the redis session key.
    let sss_token = {
        let mut claims = Claims::new()?;

        claims.add_additional(SSS_PUB_TK.uuid_key, json!(sss_uuid))?;

        if remember_me {
            claims.add_additional(SSS_PUB_TK.user_id_key, json!(user_id))?;
            claims.add_additional(SSS_PUB_TK.role_key, json!(role))?;
        }

        // Use the 256 bit secret key as the symmetric key 
        let sk = SymmetricKey::<V4>::from(settings.secret.secret_key.as_bytes())?;

        local::encrypt(
            &sk,
            &claims,
            None,
            Some(settings.secret.hmac_secret.as_bytes()),
        )?
    };

    tracing::debug!(target: "backend", "Finished.");
    Ok(sss_token)
}

/// Retrieves the session UUID from the session uuid token, and
/// uses it to retrieve the session token from redis, where the
/// key is the session key prefix plus the UUID.
/// Returns the user id.
#[tracing::instrument(name = "Verify PASETO token for session uuid", skip(redis_pool, db))]
pub async fn verify_session_token(
    sss_pub_token: String,
    db: &mongodb::Database,
    redis_pool: &deadpool_redis::Pool,
) -> Result<UserSession> {
    let settings = crate::settings::get_settings().expect("Failed to read settings.");

    let claims = get_token_claims(&settings, sss_pub_token)?;

    let sss_uuid_claim = get_claim(&claims, SSS_PUB_TK.uuid_key);
    let sss_uuid: Uuid = serde_json::from_value(sss_uuid_claim.clone())?;

    let mut redis_conn = if let Ok(conn) = get_redis_conn(redis_pool).await {
        conn
    } else {
        bail!(types::error::Redis::ConnError("Failed to obtain redis connection".into()));
    };

    let redis_key = format!("{}{}", SESSION_KEY_PREFIX, sss_uuid);
    let sss_data_token: Option<String> = redis_conn.get(redis_key.clone()).await?;

    if let Some(sss_data_token) = sss_data_token {
        redis_conn.expire::<_, ()>(redis_key, (settings.secret.session_token_expiration * 60) as i64).await?;

        let claims = get_token_claims(&settings, sss_data_token)?;

        let session_claim = get_claim(&claims, SSS_DATA_TK.session_key);
        let session: UserSession = serde_json::from_value(session_claim.clone())?;

        Ok(session)
    } else {
        let user_id_claim = claims.get_claim(SSS_PUB_TK.user_id_key);
        let role_claim = claims.get_claim(SSS_PUB_TK.role_key);

        if let (Some(user_id_claim), Some(role_claim)) = (user_id_claim, role_claim) {
            let user_id: ObjectId = serde_json::from_value(user_id_claim.clone())?;
            let role: Role = serde_json::from_value(role_claim.clone())?;

            let user =
                utils::database::users::get_user(db, user_id)
                .await?
                .expect("Failed to find user in database.");

            let redis_key = format!("{}{}", SESSION_KEY_PREFIX, sss_uuid);
            let redis_token = build_sss_data_token(&settings, user.clone(), role).await?;

            redis_conn.set_ex::<_, _, ()>(redis_key, redis_token, settings.secret.session_token_expiration * 60).await?;

            let user_session = UserSession::try_from(user, role)?;

            Ok(user_session)
        } else {
            bail!(crate::types::error::Redis::SessionExpired(
                "The session is expired and no user id was found for renewal".into()
            ));
        }
    }
}

// Build the redis token containing the user session data.
async fn build_sss_data_token(
    settings: &crate::settings::Settings,
    user: User,
    role: Role,
) -> Result<String> {
    let mut claims = Claims::new()?;

    let session = match UserSession::try_from(user, role) {
        Ok(session) => session,
        Err(e) => bail!(types::error::BadRequest::NonexistentRole(format!("{}", e)))
    };

    claims.add_additional(SSS_DATA_TK.session_key, json!(session))?;

    // Use the 256 bit secret key as the symmetric key 
    let sk = SymmetricKey::<V4>::from(settings.secret.secret_key.as_bytes())?;

    Ok(local::encrypt(
        &sk,
        &claims,
        None,
        Some(settings.secret.hmac_secret.as_bytes()),
    )?)
}

/// Retrieves the session UUID from the session uuid token, and
/// uses it to delete the session token from redis, where the
/// key is the session key prefix plus the UUID.
#[tracing::instrument(name = "Revoke PASETO token for session uuid", skip(redis_pool))]
pub async fn revoke_session_token(
    sss_pub_token: String,
    redis_pool: &deadpool_redis::Pool,
) -> Result<()> {
    let settings = crate::settings::get_settings().expect("Failed to read settings.");

    let claims = get_token_claims(&settings, sss_pub_token)?;
    let sss_uuid_claim = get_claim(&claims, SSS_PUB_TK.uuid_key);

    let sss_uuid: Uuid = serde_json::from_value(sss_uuid_claim.clone())?;

    let mut redis_conn = if let Ok(conn) = get_redis_conn(redis_pool).await {
        conn
    } else {
        bail!(types::error::Redis::ConnError("Failed to obtain redis connection".into()));
    };
    
    redis_conn.del::<_, ()>(format!("{}{}", SESSION_KEY_PREFIX, sss_uuid)).await?;

    Ok(())
}

/// Issues a PASETO token to a user for email confirmation operations. 
/// The token has the user's id encoded. A session_key is also encoded.
/// This key is used to destroy the token in redis as soon as it's been verified.
/// Depending on its usage, the token's TTL is at most an hour.
/// The token in redis is simply the email key prefix plus 128 bytes of random data.
/// Returns the PASETO token.
#[tracing::instrument(name = "Issue PASETO token for email confirmation", skip(redis_pool))]
pub async fn issue_email_token(
    user_id: ObjectId,
    redis_pool: &deadpool_redis::Pool,
    is_for_password_change: Option<bool>,
) -> Result<String> {
    // Just generate 128 bytes of random data for the session key
    // from something that is cryptographically secure (rand::CryptoRng).
    //
    // A random value is not neccesarily needed, but you'll want something
    // that is sufficiently not able to be guessed (you don't want someone getting
    // an old token that is supposed to not be live, and being able to get a valid
    // token from that). 
    let email_key: String = {
        let mut buf = [0_u8; 128];
        OsRng.fill_bytes(&mut buf);
        hex::encode(buf)
    };

    let redis_key = {
        if is_for_password_change.is_some() {
            format!(
                "{}{}is_for_password_change",
                EMAIL_KEY_PREFIX, email_key
            )
        } else {
            format!("{}{}", EMAIL_KEY_PREFIX, email_key)
        }
    };

    let mut redis_conn = if let Ok(conn) = get_redis_conn(redis_pool).await {
        conn
    } else {
        bail!(types::error::Redis::ConnError("Failed to obtain redis connection".into()))
    };

    redis_conn
        .set::<_, _, ()>(
            redis_key.clone(),
            // Since we only validate that the key exists
            // to indicate the session is "live", it can have any value
            ""
        )
        .await
        .map_err(|e| {
            tracing::error!(target: "redis", "Error setting email confirmation token: {}", e);
            e
        })?;

    let settings = crate::settings::get_settings().expect("Cannot load settings.");
    let current_date_time = chrono::Local::now();
    // For redis expiration
    let time_to_live = {
        if is_for_password_change.is_some() {
            chrono::Duration::hours(1)
        } else {
            chrono::Duration::minutes(settings.secret.email_token_expiration as i64)
        }
    };

    // For claims expiration
    let dt = current_date_time + time_to_live;

    redis_conn
        .expire::<_, ()>(
            redis_key.clone(),
            time_to_live.num_seconds().try_into().unwrap()
        )
        .await
        .map_err(|e| {
            tracing::event!(target: "backend", tracing::Level::ERROR, "RedisError (expiry): {}", e);
            e
        })?;

    let mut claims = Claims::new().unwrap();
    // Set custom expiration, default is 1 hour
    claims.expiration(&dt.to_rfc3339()).unwrap();
    claims
        .add_additional(EMAIL_TK.user_id_key, json!(user_id.to_string()))
        .unwrap();
    claims
        .add_additional(EMAIL_TK.email_key, json!(email_key))
        .unwrap();

    // Use the 256 bit secret key as the symmetric key 
    let sk = SymmetricKey::<V4>::from(settings.secret.secret_key.as_bytes()).unwrap();

    Ok(local::encrypt(
        &sk,
        &claims,
        None,
        Some(settings.secret.hmac_secret.as_bytes()),
    ).unwrap())
}

/// Verifies and destroys an email confirmation token.
/// The token is destroyed in redis immediately after it has successfully been
/// verified and all encoded data extracted.
/// Returns the user id.
#[tracing::instrument(name = "Verify PASETO token for email confirmation", skip(token, redis_pool))]
pub async fn verify_email_token(
    token: String,
    redis_pool: &deadpool_redis::Pool,
    is_password: Option<bool>
) -> Result<ObjectId> {
    let settings = crate::settings::get_settings().expect("Cannot read settings.");
    
    let claims = get_token_claims(&settings, token)?;

    let uid_claim = get_claim(&claims, EMAIL_TK.user_id_key);
    let uid: ObjectId = serde_json::from_value(uid_claim.clone())?;

    let email_key_claim = get_claim(&claims, EMAIL_TK.email_key);
    let email_key: String = serde_json::from_value(email_key_claim.clone())?;

    let redis_key = {
        if is_password.is_some() {
            format!(
                "{}{}is_for_password_change",
                EMAIL_KEY_PREFIX, email_key
            )
        } else {
            format!("{}{}", EMAIL_KEY_PREFIX, email_key)
        }
    };

    let mut redis_conn = if let Ok(conn) = get_redis_conn(redis_pool).await {
        conn
    } else {
        bail!(types::error::Redis::ConnError("Failed to obtain redis connection.".into()));
    };

    if redis_conn
        .get::<_, Option<String>>(redis_key.clone())
        .await
        .map_err(|e| anyhow!(format!("{}", e)))?
        .is_none()
    {
        bail!("Token has been used or expired.".to_string())
    }

    redis_conn.del::<_, ()>(redis_key).await?;

    Ok(uid)
}

/// Issues a PASETO token to a user for login role selection.
/// The token has the role selection session id (`roleselect_sss`) key encoded.
/// This key is used to destroy the token in redis as soon as it's been verified.
/// The token in redis is simply the roleselect key prefix plus 128 bytes of random data.
/// Returns the PASETO token.
#[tracing::instrument(
    name = "Issue PASETO token for role selection",
    skip(redis_pool, user, remember_me)
)]
pub async fn issue_roleselect_token(
    redis_pool: &deadpool_redis::Pool,
    user: User,
    remember_me: bool,
) -> Result<String> {
    // Just generate 128 bytes of random data for the session key
    // from something that is cryptographically secure (rand::CryptoRng).
    //
    // A random value is not neccesarily needed, but you'll want something
    // that is sufficiently not able to be guessed (you don't want someone getting
    // an old token that is supposed to not be live, and being able to get a valid
    // token from that). 
    let roleselect_key: String = {
        let mut buf = [0_u8; 128];
        OsRng.fill_bytes(&mut buf);
        hex::encode(buf)
    };

    let settings = crate::settings::get_settings().expect("Cannot load settings.");

    let redis_key = format!("{}{}", ROLESELECT_KEY_PREFIX, roleselect_key);
    let rolesel_data_token = build_roleselect_data_token(&settings, user, remember_me).await?;

    let mut redis_conn = if let Ok(conn) = get_redis_conn(redis_pool).await {
        conn
    } else {
        bail!(types::error::Redis::ConnError("Failed to obtain redis connection".into()))
    };

    redis_conn.set::<_, _, ()>(redis_key.clone(), rolesel_data_token)
        .await
        .map_err(|e| {
            tracing::error!(target: "redis", "Error setting role selection token: {}", e);
            e
        })?;

    let current_date_time = chrono::Local::now();
    // For redis expiration
    let time_to_live = chrono::Duration::minutes(settings.secret.roleselect_token_expiration as i64);

    // For claims expiration
    let dt = current_date_time + time_to_live;

    redis_conn
        .expire::<_, ()>(
            redis_key.clone(),
            time_to_live.num_seconds().try_into().unwrap()
        )
        .await
        .map_err(|e| {
            tracing::event!(target: "backend", tracing::Level::ERROR, "RedisError (expiry): {}", e);
            e
        })?;

    let mut claims = Claims::new().unwrap();
    // Set custom expiration.
    claims.expiration(&dt.to_rfc3339()).unwrap();
    claims
        .add_additional(ROLESEL_PUB_TK.roleselect_key, json!(roleselect_key))
        .unwrap();

    // Use the 256 bit secret key as the symmetric key 
    let sk = SymmetricKey::<V4>::from(settings.secret.secret_key.as_bytes()).unwrap();

    Ok(local::encrypt(
        &sk,
        &claims,
        None,
        Some(settings.secret.hmac_secret.as_bytes()),
    ).unwrap())
}

/// Verifies and destroys an email confirmation token.
/// The token is destroyed in redis immediately after it has successfully been
/// verified and all encoded data extracted.
/// Returns the user id.
#[tracing::instrument(name = "Verify PASETO token for role selection", skip(rolesel_pub_token, redis_pool))]
pub async fn verify_roleselect_token(
    rolesel_pub_token: String,
    redis_pool: &deadpool_redis::Pool,
) -> Result<(User, bool)> {
    let settings = crate::settings::get_settings().expect("Cannot load settings.");

    let claims = get_token_claims(&settings, rolesel_pub_token)?;

    let roleselect_key_claim  = get_claim(&claims, ROLESEL_PUB_TK.roleselect_key);
    let roleselect_key: String = serde_json::from_value(roleselect_key_claim.clone())?;

    let redis_key = format!("{}{}", ROLESELECT_KEY_PREFIX, roleselect_key);

    let mut redis_conn = if let Ok(conn) = get_redis_conn(redis_pool).await {
        conn
    } else {
        bail!(types::error::Redis::ConnError("Failed to obtain redis connection.".into()));
    };

    let rolesel_data_token: Option<String> = redis_conn.get(redis_key.clone()).await?;

    if let Some(rolesel_data_token) = rolesel_data_token {
        let claims = get_token_claims(&settings, rolesel_data_token)?;

        let user_claim = get_claim(&claims, ROLESEL_DATA_TK.user_key);
        let user: User = serde_json::from_value(user_claim.clone())?;
 
        let remember_me_claim = get_claim(&claims, ROLESEL_DATA_TK.remember_me_key);
        let remember_me: bool = serde_json::from_value(remember_me_claim.clone())?;

        redis_conn.del::<_, ()>(redis_key).await?;

        Ok((user, remember_me))
    } else {
        bail!("Role selection session expired.")
    }
}

// Build the redis token containing the role selection session data.
async fn build_roleselect_data_token(
    settings: &crate::settings::Settings,
    user: User,
    remember_me: bool
) -> Result<String> {
    // Build the redis token containing the user data.
    let mut claims = Claims::new()?;

    claims.add_additional(ROLESEL_DATA_TK.user_key, json!(user))?;
    claims.add_additional(ROLESEL_DATA_TK.remember_me_key, remember_me)?;

    // Use the 256 bit secret key as the symmetric key 
    let sk = SymmetricKey::<V4>::from(settings.secret.secret_key.as_bytes())?;

    Ok(local::encrypt(
        &sk,
        &claims,
        None,
        Some(settings.secret.hmac_secret.as_bytes()),
    )?)
}

pub fn get_token_claims(settings: &crate::settings::Settings, token: String) -> Result<Claims> {
    let sk = SymmetricKey::<V4>::from(settings.secret.secret_key.as_bytes()).unwrap();

    let validation_rules = ClaimsValidationRules::new();

    let untrusted_token = UntrustedToken::<Local, V4>::try_from(&token)
        .map_err(|e| anyhow!(format!("TokenValidation: {}", e)))?;

    let trusted_token = local::decrypt(
        &sk,
        &untrusted_token,
        &validation_rules,
        None,
        Some(settings.secret.hmac_secret.as_bytes()),
    )
    .map_err(|e| anyhow!(format!("PASETO: {}", e)))?;

    let claims = trusted_token.payload_claims().unwrap();

    Ok(claims.clone())
}

fn get_claim<'a>(claims: &'a Claims, str: &str) -> &'a serde_json::value::Value {
    let value = claims.get_claim(str)
        .expect(format!("Could not find `{}` in claims.", str).as_str());

    value
}