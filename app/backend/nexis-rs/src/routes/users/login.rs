use crate::prelude::*;
use crate::responses;
use anyhow::Result;
use types::{ mongodb::IsCollection, requests::{ self, LoginUser }, SSS_COOKIE_NAME };
use utils::tokens::{ verify_roleselect_token, issue_session_token };

const USER_NOT_FOUND_MSG: &'static str = "A user with these details does not exist. If you registered with these details, ensure you activated your account by clicking on the link sent to your e-mail address.";

#[tracing::instrument(
    name = "Logging a user in",
    skip(req, db, login, redis_pool),
    fields(
        user_identifier = %login.identifier,
        remember_me = %login.remember_me
    )
)]
#[actix_web::post("/login")]
async fn login_user(
    req: HttpRequest,
    login: web::Json<LoginUser>,
    db: web::Data<mongodb::Database>,
    redis_pool: web::Data<deadpool_redis::Pool>
) -> HttpResponse {
    tracing::info!(target: "backend", "Accessing LOGIN.");

    if req.cookie(SSS_COOKIE_NAME).is_some() {
        let sss_pub_token = req.cookie(SSS_COOKIE_NAME).unwrap().value().to_string();
        if let Ok(_) = utils::verify_session_token(sss_pub_token, &db, &redis_pool).await {
            return HttpResponse::Ok().json("You are already logged in.");
        }
    }

    match get_user_who_is_active(db.get_ref(), &login.identifier).await {
        Ok(user) => {
            let password_hash = user.password.clone();
            let password = login.password.clone();

            let verify_result = tokio::task::spawn_blocking(move || {
                utils::verify_password(password_hash, password)
            })
            .await
            .expect("Unable to unwrap JoinError.");

            match verify_result.await {
                Ok(()) => {
                    let available_roles = user.get_roles();

                    if available_roles.len() > 1 {
                        match utils::issue_roleselect_token(&redis_pool, user, login.remember_me).await {
                            Ok(token) => {
                                HttpResponse::Ok().json(responses::RoleSelect {
                                    available_roles,
                                    token,
                                })
                            }
                            Err(e) => {
                                tracing::error!(target: "backend", "An unexpected error occurred: {}", e);
                                HttpResponse::InternalServerError().finish()
                            }
                        }
                    }
                    else {
                        let (email, name) = (user.email.clone(), user.name.clone());
                        let role = available_roles[0];

                        let sss_pub_token = match utils::issue_session_token(user, role, login.remember_me, &redis_pool).await {
                            Ok(token) =>
                                token,
                            Err(e) => {
                                tracing::error!(target: "backend", "An unexpected error occurred: {}", e);
                                return HttpResponse::InternalServerError().finish();
                            }
                        };

                        let session_cookie = {
                            let mut cookie = Cookie::build(SSS_COOKIE_NAME, sss_pub_token.to_string())
                                .path("/")
                                .http_only(true)
                                .finish();
                            if login.remember_me {
                                cookie.make_permanent();
                            }
                            cookie
                        };
                        
                        tracing::debug!(target: "backend", "Finished.");
                        HttpResponse::Ok()
                            .cookie(session_cookie)
                            .json(responses::User {
                                email,
                                name,
                                role,
                                client: None,
                                employee: None,
                                admin: None,
                            }
                        )
                    }
                }
                Err(e) => {
                    tracing::error!(target: "backend", "Wrong password: {:#?}", e);
                    HttpResponse::NotFound().json(responses::Error::simple(USER_NOT_FOUND_MSG))
                }
            }
        }
        Err(e) => {
            tracing::error!(target: "backend", "User not found: {:#?}", e);
            HttpResponse::NotFound().json(responses::Error::simple(USER_NOT_FOUND_MSG))
        }
    }
}

#[tracing::instrument(name = "Getting a user from DB.", skip(db, identifier),fields(user_identifier = %identifier))]
pub async fn get_user_who_is_active(
    db: &mongodb::Database,
    identifier: &String,
) -> Result<types::mongodb::User> {
    let users_coll: Collection<types::mongodb::User> = db.collection(types::mongodb::User::coll_name());
    let res = users_coll.find_one(
        doc! {
            "$or": [
                { "email": identifier },
                { "username": identifier }
            ],
            "isActive": true
        }
    ).await;

    match res {
        Ok(res) =>
            if let Some(user) = res {
                tracing::debug!(target: "backend", "Finished.");
                Ok(user)
            } else {
                tracing::error!(target: "mongodb", "User not found in DB. Identifier: {}", identifier);
                Err(anyhow!("User not found in DB."))
            },
        Err(e) => {
            tracing::error!(target: "mongodb", "Failed to query the mongodb database: {:#?}", e);
            Err(anyhow!(e))
        }
    }
}

#[tracing::instrument(
    name = "Logging a user in with role selection",
    skip(login, redis_pool),
    fields(
        role = %login.role,
    )
)]
#[actix_web::post("/role-login")]
async fn role_login(
    login: web::Json<requests::RoleLoginUser>,
    redis_pool: web::Data<deadpool_redis::Pool>
) -> HttpResponse {
    tracing::info!(target: "backend", "Accessing LOGIN role selection.");

    let role = login.role;
    let rolesel_pub_token = &login.rolesel_pub_token;

    let res = verify_roleselect_token(rolesel_pub_token.to_string(), &redis_pool).await;

    match res {
        Ok((user, remember_me)) => {
            let session_cookie = {
                use types::error::BadRequest;

                let sss_pub_token = match issue_session_token(user.clone(), role, remember_me, &redis_pool).await {
                    Ok(token) => token,
                    Err(e) if e.is::<types::error::BadRequest>() => {
                        if let Some(BadRequest::NonexistentRole(e)) = e.downcast_ref::<BadRequest>() {
                            return HttpResponse::BadRequest().json(responses::Error::from_str(e.to_string()))
                        } else {
                            tracing::error!(target: "backend", "{}", e);
                            return HttpResponse::InternalServerError().finish();
                        }
                    }
                    Err(e) => {
                        tracing::error!(target: "backend", "{}", e);
                        return HttpResponse::InternalServerError().finish();
                    }
                };

                let mut cookie = Cookie::build(SSS_COOKIE_NAME, sss_pub_token)
                    .path("/")
                    .secure(true)
                    .http_only(true)
                    .finish();
                if remember_me {
                    cookie.make_permanent();
                }

                cookie
            };

            HttpResponse::Ok()
                .cookie(session_cookie)
                .json(responses::User {
                    email: user.email,
                    name: user.name,
                    role,
                    client: None,
                    employee: None,
                    admin: None,
                })
        }
        Err(e) => {
            tracing::error!(target: "backend", "{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}