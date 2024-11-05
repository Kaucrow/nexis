use crate::prelude::*;
use crate::responses;
use anyhow::Result;
use crate::types::mongodb::User;

#[derive(Deserialize)]
pub struct Parameters {
    token: String,
}

#[tracing::instrument(
    name = "Activating a new user",
    skip(parameters, db, redis_pool)
)]
#[actix_web::put("/register/verify")]
pub async fn confirm(
    parameters: web::Query<Parameters>,
    db: web::Data<mongodb::Database>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResponse {
    let settings = crate::settings::get_settings().expect("Failed to read settings.");
    tracing::event!(target: "backend", tracing::Level::INFO, "Token {:#?}", parameters.token);
 
    let user_id = match crate::utils::verify_email_token(
        parameters.token.clone(),
        &redis_pool,
        None
    )
    .await
    {
        Ok(token) => token,
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "{:#?}", e);

            // If the token has expired or has already been used
            return HttpResponse::SeeOther().insert_header((
                    http::header::LOCATION,
                    format!("{}/auth/regenerate-token", settings.frontend_url)
                ))
                .finish();
        }
    };

    match activate_new_user(&db, user_id).await {
        Ok(_) => {
            tracing::event!(target: "backend", tracing::Level::INFO, "New user was activated successfully.");

            // If the user is activated successfully
            HttpResponse::Ok().json(responses::Success::new("User activated successfully."))
        }
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "Cannot activate account: {}", e);

            // If the account cannot be activated
            HttpResponse::SeeOther()
                .insert_header((
                    http::header::LOCATION,
                    format!("{}/auth/error?reason={e}", settings.frontend_url),
                ))
                .finish()
        }
    }
}

#[tracing::instrument(name = "Mark a user active", skip(db), fields(
    new_user_user_id = %user_id
))]
pub async fn activate_new_user(
    db: &mongodb::Database,
    user_id: ObjectId, 
) -> Result<()> {
    let users_coll: Collection<User> = db.collection("user");

    let query = doc! { "_id": user_id };
    let update = doc! { "$set": { "isActive": true }};

    let res = users_coll.update_one(query, update).await?;

    if res.matched_count == 0 {
        tracing::error!(target: "mongodb", "No user found with the given id.");
    } else {
        tracing::debug!(target: "mongodb", "Set isActive to true on user.");
    }

    Ok(())
}