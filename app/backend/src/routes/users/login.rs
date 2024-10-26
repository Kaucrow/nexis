use crate::prelude::*;
use anyhow::Result;
use crate::types::{ User, LoginUser };

const USER_NOT_FOUND_MSG: &'static str = "A user with these details does not exist. If you registered with these details, ensure you activated your account by clicking on the link sent to your e-mail address.";

#[tracing::instrument(name = "Logging a user in", skip(db, user, redis_pool), fields(user_email = %user.email))]
#[actix_web::post("/login")]
async fn login_user(
    user: web::Json<LoginUser>,
    db: web::Data<mongodb::Database>,
    redis_pool: web::Data<deadpool_redis::Pool>
) -> HttpResponse {
    tracing::info!(target: "backend", "Accessing LOGIN.");
    match get_user_who_is_active(db.get_ref(), &user.email).await {
        Ok(db_user) => {
            let password_hash = db_user.password.clone();
            let password = user.password.clone();

            let verify_result = tokio::task::spawn_blocking(move || {
                crate::utils::verify_password(password_hash, password)
            })
            .await
            .expect("Unable to unwrap JoinError.");

            match verify_result.await {
                Ok(()) => {
                    let sss_uuid_token = match crate::utils::issue_session_token(db_user.id, &redis_pool).await {
                        Ok(token) =>
                            token,
                        Err(e) if e.is::<crate::types::error::Redis>() =>
                            return HttpResponse::InternalServerError().finish(),
                        Err(e) => {
                            tracing::error!(target: "backend", "An unexpected error occurred: {}", e);
                            return HttpResponse::InternalServerError().finish();
                        }
                    };

                    HttpResponse::Ok()
                        .cookie(Cookie::build("session_uuid", sss_uuid_token.to_string())
                            .path("/")
                            .http_only(true)
                            .finish()
                        )
                        .json(crate::types::UserResponse {
                            email: db_user.email,
                            name: db_user.name,
                            client: None,
                            employee: None,
                            admin: None,
                        })
                }
                Err(e) => {
                    tracing::event!(target: "backend", tracing::Level::ERROR, "Wrong password: {:#?}", e);
                    HttpResponse::NotFound().json(crate::types::ErrorResponse {
                        error: USER_NOT_FOUND_MSG.to_string()
                    })
                }
            }
        }
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "User not found: {:#?}", e);
            HttpResponse::NotFound().json(crate::types::ErrorResponse {
                error: USER_NOT_FOUND_MSG.to_string()
            })
        }
    }
}

#[tracing::instrument(name = "Getting a user from DB.", skip(db, email),fields(user_email = %email))]
pub async fn get_user_who_is_active(
    db: &mongodb::Database,
    email: &String,
) -> Result<crate::types::User> {
    let users_coll: Collection<User> = db.collection("user");
    let res = users_coll.find_one(
        doc! { "email": email, "isActive": true }
    ).await;

    match res {
        Ok(res) =>
            if let Some(user) = res {
                Ok(user)
            } else {
                tracing::event!(target: "sqlx", tracing::Level::ERROR, "User not found in DB.");
                Err(anyhow!("User not found in DB."))
            },
        Err(e) => {
            tracing::event!(target: "mongodb", tracing::Level::ERROR, "Failed to query the mongodb database: {:#?}", e);
            Err(anyhow!(e))
        }
    }
}