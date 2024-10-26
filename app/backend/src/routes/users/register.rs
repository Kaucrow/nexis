use crate::prelude::*;
use crate::{
    utils::{
        auth::password::hash,
        send_multipart_email,
    },
    database::insert_created_user_into_db,
    types::{
        NewUser,
        SuccessResponse,
        ErrorResponse,
    },
};

#[tracing::instrument(name = "Adding a new user",
skip(db, new_user, redis_pool),
fields(
    new_user_email = %new_user.email,
    new_user_name = %new_user.name,
))]
#[actix_web::post("/register")]
pub async fn register_user(
    new_user: web::Json<NewUser>,
    db: web::Data<mongodb::Database>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResponse {
    tracing::event!(target: "backend", tracing::Level::INFO, "Reached /users/register");
    // open redis connection
    let mut redis_con = redis_pool
        .get()
        .await
        .map_err(|e| {
            tracing::event!(target: "backend", tracing::Level::ERROR, "{}", e);
            actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                error: "We cannot activate your account at the moment".to_string(),
            })
        })
        .expect("Redis connection cannot be obtained.");

    let hashed_password = hash(new_user.0.password.as_bytes()).await;
    
    let create_new_user = NewUser {
        password: hashed_password,
        email: new_user.0.email,
        username: new_user.0.username,
        name: new_user.0.name,
        client: new_user.0.client,
        employee: new_user.0.employee,
        admin: new_user.0.admin,
    };

    let user_id = match insert_created_user_into_db(db.get_ref(), create_new_user.clone()).await {
        Ok(id) => id,
        Err(e) => {
            // TODO: Handle a repeated username or email
            tracing::event!(target: "mongodb", tracing::Level::ERROR, "Failed to insert user into DB: {:#?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    send_multipart_email(
        "Actix Login Sign Up".to_string(),
        user_id,
        create_new_user.email,
        create_new_user.name,
        "verification_email.html",
        &mut redis_con
    )
    .await
    .unwrap();

    tracing::event!(target: "backend", tracing::Level::INFO, "User created successfully.");
    actix_web::HttpResponse::Ok().json(SuccessResponse {
        message: "Your account was created successfully. Check your email address to activate your account as we just sent you an activation link. Ensure you activate your account before the link expires".to_string(),
    })
}