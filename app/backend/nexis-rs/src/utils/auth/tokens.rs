use crate::prelude::*;
use anyhow::Result;
use crate::database::get_redis_conn;
use argon2::password_hash::rand_core::{ OsRng, RngCore };
use core::convert::TryFrom;
use deadpool_redis::redis::AsyncCommands;
use hex;
use serde_json::json;

/// Store the session key prefix as a const so it can't be typo'd anywhere it's used.
const SESSION_KEY_PREFIX: &str = "session_";
/// Store the email key prefix as a const so it can't be typo'd anywhere it's used.
const EMAIL_KEY_PREFIX: &str = "email_";

/// Issues a PASETO token to a user for storing the session.
/// Returns the session UUID token which should be set as a cookie,
/// and sets a key-value pair in Redis where this UUID is the key
/// and the session token is the value. This token has the user's id encoded.
#[tracing::instrument(name = "Issue PASETO token for session uuid", skip(redis_pool))]
pub async fn issue_session_token(
    user_id: ObjectId,
    remember_me: bool,
    redis_pool: &deadpool_redis::Pool,
) -> Result<String> {
    let settings = crate::settings::get_settings().expect("Cannot read settings.");

    let sss_uuid = Uuid::new_v4();

    let mut redis_conn = if let Ok(conn) = get_redis_conn(redis_pool).await {
        conn
    } else {
        bail!(crate::types::error::Redis::ConnError("Failed to obtain redis connection.".into()));
    };

    let redis_key = format!("{}{}", SESSION_KEY_PREFIX, sss_uuid);

    // Build the redis token containing the user id.
    let redis_token = build_redis_token(&settings, user_id).await?;

    redis_conn.set_ex::<_, _, ()>(redis_key, redis_token, settings.secret.session_token_expiration * 60).await?;

    // Build the session token to be set as a cookie, containing the UUID for the redis session key.
    let sss_token = {
        let mut claims = Claims::new()?;

        claims.add_additional("session_uuid", json!(sss_uuid))?;

        if remember_me {
            claims.add_additional("user_id", json!(user_id))?;
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

    Ok(sss_token)
}

/// Retrieves the session UUID from the session uuid token, and
/// uses it to retrieve the session token from redis, where the
/// key is the session key prefix plus the UUID.
/// Returns the user id.
#[tracing::instrument(name = "Verify PASETO token for session uuid", skip(redis_pool, db))]
pub async fn verify_session_token(
    sss_uuid_token: String,
    db: &mongodb::Database,
    redis_pool: &deadpool_redis::Pool,
) -> Result<ObjectId> {
    let settings = crate::settings::get_settings().expect("Failed to read settings.");
    let sk = SymmetricKey::<V4>::from(settings.secret.secret_key.as_bytes()).unwrap();
    let validation_rules = ClaimsValidationRules::new();

    let untrusted_token = UntrustedToken::<Local, V4>::try_from(&sss_uuid_token)?;
    let trusted_token = local::decrypt(
        &sk,
        &untrusted_token,
        &validation_rules,
        None,
        Some(settings.secret.hmac_secret.as_bytes())
    )?;

    let claims = trusted_token.payload_claims().expect("Failed to get token claims");
    let sss_uuid_claim = claims.get_claim("session_uuid").expect("");

    let sss_uuid: Uuid = serde_json::from_value(sss_uuid_claim.clone())?;

    let mut redis_conn = if let Ok(conn) = get_redis_conn(redis_pool).await {
        conn
    } else {
        bail!(types::error::Redis::ConnError("Failed to obtain redis connection".into()));
    };

    let redis_key = format!("{}{}", SESSION_KEY_PREFIX, sss_uuid);
    let sss_token: Option<String> = redis_conn.get(redis_key.clone()).await?;

    if let Some(sss_token) = sss_token {
        redis_conn.expire::<_, ()>(redis_key, (settings.secret.session_token_expiration * 60) as i64).await?;

        let validation_rules = ClaimsValidationRules::new();
        let untrusted_token = UntrustedToken::<Local, V4>::try_from(&sss_token)?;
        let trusted_token = local::decrypt(
            &sk,
            &untrusted_token,
            &validation_rules,
            None,
            Some(settings.secret.hmac_secret.as_bytes())
        )?;

        let claims = trusted_token.payload_claims().expect("Failed to get token claims");

        let uid = serde_json::to_value(claims.get_claim("user_id").unwrap())?;

        match serde_json::from_value::<String>(uid) {
            Ok(uid) => match ObjectId::parse_str(uid) {
                Ok(uid) => {
                    return Ok(uid)
                }
                Err(e) => bail!(format!("{}", e)),
            },
            Err(e) => bail!(format!("{}", e)), 
        }
    } else {
        let user_id_claim = claims.get_claim("user_id");
        if let Some(user_id_claim) = user_id_claim {
            let user_id: ObjectId = serde_json::from_value(user_id_claim.clone())?;
            let user =
                crate::database::get_db_user(db, user_id)
                .await?
                .expect("Failed to find user in database.");

            let redis_key = format!("{}{}", SESSION_KEY_PREFIX, sss_uuid);
            let redis_token = build_redis_token(&settings, user.id).await?;

            redis_conn.set_ex::<_, _, ()>(redis_key, redis_token, settings.secret.session_token_expiration * 60).await?;

            Ok(user.id)
        } else {
            bail!(crate::types::error::Redis::SessionExpired(
                "The session is expired and no user id was found for renewal".into()
            ));
        }
    }
}


/// Retrieves the session UUID from the session uuid token, and
/// uses it to delete the session token from redis, where the
/// key is the session key prefix plus the UUID.
#[tracing::instrument(name = "Revoke PASETO token for session uuid", skip(redis_pool))]
pub async fn revoke_session_token(
    sss_uuid_token: String,
    redis_pool: &deadpool_redis::Pool,
) -> Result<()> {
    let settings = crate::settings::get_settings().expect("Failed to read settings.");
    let sk = SymmetricKey::<V4>::from(settings.secret.secret_key.as_bytes()).unwrap();
    let validation_rules = ClaimsValidationRules::new();

    let untrusted_token = UntrustedToken::<Local, V4>::try_from(&sss_uuid_token)?;
    let trusted_token = local::decrypt(
        &sk,
        &untrusted_token,
        &validation_rules,
        None,
        Some(settings.secret.hmac_secret.as_bytes())
    )?;

    let claims = trusted_token.payload_claims().expect("Failed to get token claims");
    let sss_uuid_claim = claims.get_claim("session_uuid").expect("Failed to get `session_uuid` claim in token.");

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
pub async fn issue_confirmation_token(
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
        .add_additional("user_id", json!(user_id.to_string()))
        .unwrap();
    claims
        .add_additional("email_key", json!(email_key))
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
pub async fn verify_confirmation_token(
    token: String,
    redis_pool: &deadpool_redis::Pool,
    is_password: Option<bool>
) -> Result<ObjectId> {
    let settings = crate::settings::get_settings().expect("Cannot read settings.");
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

    let uid_claim = claims.get_claim("user_id").expect("Failed to get `user_id` claim in token.");

    let uid: ObjectId = serde_json::from_value(uid_claim.clone())?;

    let email_key_claim = claims.get_claim("email_key").unwrap();
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
        bail!(crate::types::error::Redis::ConnError("Failed to obtain redis connection.".into()));
    };

    if redis_conn
        .get::<_, Option<String>>(redis_key.clone())
        .await
        .map_err(|e| anyhow!(format!("{}", e)))?
        .is_none()
    {
        bail!("Token has been used or expired.".to_string())
    }

    redis_conn
        .del::<_, ()>(redis_key.clone())
        .await
        .map_err(|e| anyhow!(format!("{}", e)))?;

    Ok(uid)
}

async fn build_redis_token(
    settings: &crate::settings::Settings,
    user_id: ObjectId,
) -> Result<String> {
    // Build the redis token containing the user id.
    let mut claims = Claims::new()?;

    claims.add_additional("user_id", json!(user_id.to_string()))?;

    // Use the 256 bit secret key as the symmetric key 
    let sk = SymmetricKey::<V4>::from(settings.secret.secret_key.as_bytes())?;

    Ok(local::encrypt(
        &sk,
        &claims,
        None,
        Some(settings.secret.hmac_secret.as_bytes()),
    )?)
}