use crate::prelude::*;
use crate::settings::{ Environment, get_settings };
use types::{ responses, ROLESEL_PUB_TK, SSS_COOKIE_NAME, SSS_PUB_TK };
use utils::{
    get_redis_conn,
    auth::tokens::get_token_claims,
};
use anyhow::Result;

/// Store the session key prefix as a const so it can't be typo'd anywhere it's used.
const SESSION_KEY_PREFIX: &str = "session_";
/// Store the role selection key prefix as a const so it can't be typo'd anywhere it's used.
const ROLESELECT_KEY_PREFIX: &str = "roleselect_";

#[tracing::instrument(
    name = "DEBUG ENDPOINT: Getting redis session claims.",
    skip(req, redis_pool)
)]
#[actix_web::get("/redis/get-session")]
pub async fn get_redis_session(
    req: HttpRequest,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResponse {
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "development".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    if let Environment::Production = environment {
        return HttpResponse::Forbidden().json(responses::Error {
            error: "Can't access debugging endpoints on production.".to_string()
        })
    }

    let sss_pub_token =
        if let Some(sss_pub_cookie) = req.cookie(SSS_COOKIE_NAME) {
            sss_pub_cookie.value().to_string()
        } else {
            return HttpResponse::BadRequest().json(
                responses::Error { error: "Session cookie missing.".to_string() }
            );
        };

    let sss_data_claims =
        if let Ok(claims) = get_data_token_claims(sss_pub_token, SSS_PUB_TK.uuid_key, SESSION_KEY_PREFIX, &redis_pool).await {
            claims
        } else {
            return HttpResponse::InternalServerError().finish();
        };

    HttpResponse::Ok().json(format!("{:#?}", sss_data_claims))
}

#[derive(Deserialize)]
struct DebugRedisRoleselParams {
    token: String
}

#[tracing::instrument(
    name = "DEBUG ENDPOINT: Getting redis role selection claims.",
    skip(parameters, redis_pool)
)]
#[actix_web::get("/redis/get-roleselect")]
pub async fn get_redis_roleselect(
    parameters: web::Query<DebugRedisRoleselParams>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResponse {
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "development".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    if let Environment::Production = environment {
        return HttpResponse::Forbidden().json(responses::Error {
            error: "Can't access debugging endpoints on production.".to_string()
        })
    }

    let rolesel_pub_token = parameters.token.clone();

    let rolesel_data_claims =
        if let Ok(claims) =
            get_data_token_claims(
                rolesel_pub_token, ROLESEL_PUB_TK.roleselect_key, ROLESELECT_KEY_PREFIX, &redis_pool
            )
            .await 
        {
            claims
        } else {
            return HttpResponse::InternalServerError().finish();
        };

    HttpResponse::Ok().json(format!("{:#?}", rolesel_data_claims))
}

async fn get_data_token_claims(
    pub_token: String,
    claim_key: &'static str,
    redis_prefix: &'static str,
    redis_pool: &deadpool_redis::Pool
) -> Result<Claims> {
    let settings = get_settings()?;
 
    let claims = get_token_claims(&settings, pub_token)?;

    let sss_uuid_claim =
        claims.get_claim(claim_key)
        .ok_or_else(|| anyhow!(format!("Could not find `{}` claim.", claim_key)))?;

    let sss_uuid: String = serde_json::from_value(sss_uuid_claim.clone())?;

    let mut redis_conn = if let Ok(conn) = get_redis_conn(redis_pool).await {
        conn
    } else {
        bail!(crate::types::error::Redis::ConnError("Failed to obtain redis connection.".into()));
    };

    let redis_key = format!("{}{}", redis_prefix, sss_uuid);

    let data_token = redis_conn.get::<_, String>(redis_key).await?;

    let claims = get_token_claims(&settings, data_token)?;

    Ok(claims)
}