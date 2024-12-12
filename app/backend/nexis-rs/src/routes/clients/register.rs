use crate::prelude::*;
use types::{ responses, NewClient };
use utils::{
    database::{ insert_created_user_into_db, NewUser },
    auth::password::hash,
    send_multipart_email,
    get_redis_conn,
};

#[tracing::instrument(name = "Adding a new client",
skip(db, new_client, redis_pool),
fields(
    new_client_email = %new_client.email,
    new_client_name = %new_client.name,
))]
#[actix_web::post("/register")]
pub async fn register_client(
    new_client: web::Json<NewClient>,
    db: web::Data<mongodb::Database>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResponse {
    tracing::event!(target: "backend", tracing::Level::INFO, "Reached /clients/register");

    // Ensure the redis server is up before attempting to register a user.
    if let Err(_) = get_redis_conn(&redis_pool).await {
        return HttpResponse::InternalServerError().json(responses::Error::simple("Your account cannot be registered at the moment"));
    };

    let hashed_password = hash(new_client.0.password.as_bytes()).await;
    
    let mut create_new_client = new_client.0;
    create_new_client.password = hashed_password;

    let user_id = match insert_created_user_into_db(db.get_ref(), NewUser::Client(create_new_client.clone())).await {
        Ok(id) => id,
        Err(e) => {
            if let Some(e) = e.downcast_ref::<types::error::Mongodb>() {
                match e {
                    types::error::Mongodb::UserAlreadyExists(msg) => {
                        tracing::error!(target: "mongodb", msg);
                        return HttpResponse::Conflict().json(msg);
                    }
                    _ => unimplemented!()
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
        create_new_client.email,
        create_new_client.name,
        "verification_email.html",
        &redis_pool
    )
    .await
    .unwrap();

    tracing::info!(target: "backend", "User created successfully.");
    actix_web::HttpResponse::Ok().json(responses::Success::new(
        "Your account was created successfully. Check your email address to activate your account as we just sent you an activation link. Ensure you activate your account before the link expires.",
    ))
}