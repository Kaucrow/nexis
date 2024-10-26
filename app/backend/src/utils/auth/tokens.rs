use crate::prelude::*;
use anyhow::Result;
use crate::database::get_redis_conn;
use argon2::password_hash::rand_core::{ OsRng, RngCore };
use core::convert::TryFrom;
use deadpool_redis::redis::AsyncCommands;
use hex;
use serde_json::json;

/// Store the session key prefix as a const so it can't be typo'd anywhere it's used.
const SESSION_KEY_PREFIX: &str = "valid_session_key_for_";
const EMAIL_KEY_PREFIX: &str = "valid_email_key_for_";

/// Issues a PASETO token to a user for storing the session.
/// Returns the session UUID which should be set in a PASETO as a cookie,
/// and sets a key-value pair in Redis where this UUID is the key
/// and the session token is the value. This token has the user's id encoded.
#[tracing::instrument(name = "Issue PASETO token for session uuid", skip(redis_pool))]
pub async fn issue_session_token(
    user_id: ObjectId,
    redis_pool: &deadpool_redis::Pool,
) -> Result<String> {
    let settings = crate::settings::get_settings().expect("Cannot read settings.");

    let sss_uuid = Uuid::new_v4();
    let mut redis_conn = get_redis_conn(redis_pool).await?;

    let redis_key = format!("{}{}", SESSION_KEY_PREFIX, sss_uuid);

    let redis_token = {
        let mut claims = Claims::new()?;

        claims.add_additional("user_id", json!(user_id.to_string()))?;

        // Use the 256 bit secret key as the symmetric key 
        let sk = SymmetricKey::<V4>::from(settings.secret.secret_key.as_bytes())?;

        local::encrypt(
            &sk,
            &claims,
            None,
            Some(settings.secret.hmac_secret.as_bytes()),
        )?
    };

    redis_conn.set::<_, _, ()>(redis_key, redis_token).await?;

    let sss_token = {
        let mut claims = Claims::new()?;

        claims.add_additional("session_uuid", json!(sss_uuid))?;

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

/// Fetches 
/// Returns the session UUID which should be set as a cookie,
/// and sets a key-value pair in Redis where this UUID is the key
/// and the session token is the value. This token has the user's id encoded.
#[tracing::instrument(name = "Verify PASETO token for session uuid", skip(redis_pool))]
pub async fn verify_session_token(
    sss_uuid_token: String,
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

    let mut redis_conn = get_redis_conn(redis_pool).await?;

    let sss_token: String = redis_conn.get(format!("{}{}", SESSION_KEY_PREFIX, sss_uuid)).await?;

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
                Ok(uid)
            }
            Err(e) => Err(anyhow!(format!("{}", e))),
        },
        Err(e) => Err(anyhow!(format!("{}", e))), 
    }
}

/// Fetches 
/// Returns the session UUID which should be set as a cookie,
/// and sets a key-value pair in Redis where this UUID is the key
/// and the session token is the value. This token has the user's id encoded.
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

    let mut redis_conn = get_redis_conn(redis_pool).await?;
    
    redis_conn.del::<_, ()>(format!("{}{}", SESSION_KEY_PREFIX, sss_uuid)).await?;
    
    Ok(())
}

/// Issues a PASETO token to a user for email confirmation operations. 
/// The token has the user's id encoded. A session_key is also encoded.
/// This key is used to destroy the token in redis as soon as it's been verified.
/// Depending on its usage, the token issued has at most an hour to live.
/// Which means, it is destroyed after its time-to-live.
#[tracing::instrument(name = "Issue PASETO token for email confirmation", skip(redis_connection))]
pub async fn issue_confirmation_token(
    user_id: ObjectId,
    redis_connection: &mut deadpool_redis::redis::aio::MultiplexedConnection,
    is_for_password_change: Option<bool>,
) -> Result<String, redis::RedisError> {
    // I just generate 128 bytes of random data for the session key
    // from something that is cryptographically secure (rand::CryptoRng)
    //
    // You don't necessarily need a random value, but you'll want something
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

    redis_connection
        .set::<_, _, ()>(
            redis_key.clone(),
            // since we only validate that the key exists
            // to indicate the session is "live", it can have any value
            String::new(),
        )
        .await
        .map_err(|e| {
            tracing::event!(target: "backend", tracing::Level::ERROR, "RedisError (set): {}", e);
            e
        })?;

    let settings = crate::settings::get_settings().expect("Cannot load settings.");
    let current_date_time = chrono::Local::now();
    // for redis expiration
    let time_to_live = {
        if is_for_password_change.is_some() {
            chrono::Duration::hours(1)
        } else {
            chrono::Duration::minutes(settings.secret.token_expiration)
        }
    };
    // for claims expiration
    let dt = current_date_time + time_to_live;

    redis_connection
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
    // set custom expiration, default is 1 hour
    claims.expiration(&dt.to_rfc3339()).unwrap();
    claims
        .add_additional("user_id", json!(user_id.to_string()))
        .unwrap();
    claims
        .add_additional("email_key", json!(email_key))
        .unwrap();

    // use the 256 bit secret key as the symmetric key 
    let sk = SymmetricKey::<V4>::from(settings.secret.secret_key.as_bytes()).unwrap();

    Ok(local::encrypt(
        &sk,
        &claims,
        None,
        Some(settings.secret.hmac_secret.as_bytes()),
    )
    .unwrap())
}

/// Verifies and destroys a token. A token is destroyed immediately after
/// it has successfully been verified and all encoded data extracted.
/// Redis is used for such destruction.
#[tracing::instrument(name = "Verify PASETO token for email confirmation", skip(token, redis_connection))]
pub async fn verify_confirmation_token(
    token: String,
    redis_connection: &mut deadpool_redis::redis::aio::MultiplexedConnection,
    is_password: Option<bool>
) -> Result<crate::types::ConfirmationToken, String> {
    let settings = crate::settings::get_settings().expect("Cannot read settings.");
    let sk = SymmetricKey::<V4>::from(settings.secret.secret_key.as_bytes()).unwrap();

    let validation_rules = ClaimsValidationRules::new();
    let untrusted_token = UntrustedToken::<Local, V4>::try_from(&token)
        .map_err(|e| format!("TokenValidation: {}", e))?;
    let trusted_token = local::decrypt(
        &sk,
        &untrusted_token,
        &validation_rules,
        None,
        Some(settings.secret.hmac_secret.as_bytes()),
    )
    .map_err(|e| format!("Pasetor: {}", e))?;

    let claims = trusted_token.payload_claims().unwrap();

    let uid = serde_json::to_value(claims.get_claim("user_id").unwrap()).unwrap();

    tracing::debug!(target: "backend", "uid claim: {:#?}", uid);

    match serde_json::from_value::<String>(uid) {
        Ok(uid) => match ObjectId::parse_str(uid) {
            Ok(uid) => {
                let email_key =
                    // Convert to serde value to be able to get the session key from the value
                    serde_json::to_value(claims.get_claim("email_key").unwrap()).unwrap();
                let email_key = match serde_json::from_value::<String>(email_key) {
                    Ok(email_key) => email_key,
                    Err(e) => return Err(format!("{}", e)),
                };

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

                if redis_connection
                    .get::<_, Option<String>>(redis_key.clone())
                    .await
                    .map_err(|e| format!("{}", e))?
                    .is_none()
                {
                    return Err("Token has been used or expired.".to_string())
                }

                redis_connection
                    .del::<_, ()>(redis_key.clone())
                    .await
                    .map_err(|e| format!("{}", e))?;

                Ok(crate::types::ConfirmationToken { user_id: uid })
            }
            Err(e) => Err(format!("{}", e)),
        },
        Err(e) => Err(format!("{}", e)),
    }
}