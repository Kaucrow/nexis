use crate::prelude::*;
use crate::{
    responses,
    utils::{
        auth::password::hash,
        database::insert_created_user_into_db,
        send_multipart_email,
        get_redis_conn,
    },
    types::NewUser,
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

    // Ensure the redis server is up before attempting to register a user.
    if let Err(_) = get_redis_conn(&redis_pool).await {
        return HttpResponse::InternalServerError().json("Your account cannot be registered at the moment.")
    };

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
            if let Some(e) = e.downcast_ref::<types::error::Mongodb>() {
                match e {
                    types::error::Mongodb::UserAlreadyExists(msg) => {
                        tracing::error!(target: "mongodb", msg);
                        return HttpResponse::Conflict().json(msg);
                    }
                }
            } else {
                tracing::error!(target: "mongodb", "Failed to insert user into DB: {:#?}", e);
                return HttpResponse::InternalServerError().finish();
            }
        }
    };

    send_multipart_email(
        "Actix Login Sign Up".to_string(),
        user_id,
        create_new_user.email,
        create_new_user.name,
        "verification_email.html",
        &redis_pool
    )
    .await
    .unwrap();

    tracing::event!(target: "backend", tracing::Level::INFO, "User created successfully.");
    actix_web::HttpResponse::Ok().json(responses::Success {
        message: "Your account was created successfully. Check your email address to activate your account as we just sent you an activation link. Ensure you activate your account before the link expires".to_string(),
    })
}