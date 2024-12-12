use crate::prelude::*;
use super::verify_session;
use types::{ responses, NewEmployee };
use utils::{
    database::{ NewUser, insert_created_user_into_db },
    auth::password::hash,
    send_multipart_email,
    get_redis_conn,
};

#[tracing::instrument(name = "Adding a new employee",
skip(db, new_employee, redis_pool),
fields(
    new_employee_email = %new_employee.email,
    new_employee_name = %new_employee.name,
))]
#[actix_web::post("/register-employee")]
pub async fn register_employee(
    req: HttpRequest,
    new_employee: web::Json<NewEmployee>,
    db: web::Data<mongodb::Database>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResponse {
    tracing::event!(target: "backend", tracing::Level::INFO, "Reached /admins/register-employee");

    let admin_session = match verify_session(&req, &db, &redis_pool).await {
        Ok(session) => session,
        Err(e) => {
            return e;
        }
    };

    let schedule = &new_employee.schedule;
    for day in schedule.iter() {
        if !admin_session.stores.contains(&day.store) {
            return HttpResponse::Unauthorized().json(responses::Error::from_str(
                format!("You don't have the permissions required to assign an employee to `{}`", day.store)
            ));
        }
    }

    // Ensure the redis server is up before attempting to register a user.
    if let Err(_) = get_redis_conn(&redis_pool).await {
        return HttpResponse::InternalServerError().json(responses::Error::simple("The employee cannot be registered at the moment"));
    };

    let hashed_password = hash(new_employee.0.password.as_bytes()).await;
    
    let mut create_new_employee = new_employee.0;
    create_new_employee.password = hashed_password;


    let user_id = match insert_created_user_into_db(db.get_ref(), NewUser::Employee(create_new_employee.clone())).await {
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
        create_new_employee.email,
        create_new_employee.name,
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