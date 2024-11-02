use crate::prelude::*;
use anyhow::Result;
use types::{ User, LoginUser, responses, requests, SSS_COOKIE_NAME };
use utils::{auth::tokens::verify_roleselect_token, issue_session_token};

const USER_NOT_FOUND_MSG: &'static str = "A user with these details does not exist. If you registered with these details, ensure you activated your account by clicking on the link sent to your e-mail address.";

#[tracing::instrument(
    name = "Logging a user in",
    skip(req, db, user, redis_pool),
    fields(
        user_email = %user.email,
        remember_me = %user.remember_me
    )
)]
#[actix_web::post("/login")]
async fn login_user(
    req: HttpRequest,
    user: web::Json<LoginUser>,
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

    match get_user_who_is_active(db.get_ref(), &user.email).await {
        Ok(db_user) => {
            let password_hash = db_user.password.clone();
            let password = user.password.clone();

            let verify_result = tokio::task::spawn_blocking(move || {
                utils::verify_password(password_hash, password)
            })
            .await
            .expect("Unable to unwrap JoinError.");

            match verify_result.await {
                Ok(()) => {
                    let roles = db_user.get_roles();

                    if roles.len() > 1 {
                        match utils::issue_roleselect_token(&redis_pool, db_user).await {
                            Ok(token) => {
                                let roles: Vec<String> = roles.into_iter().map(|role| role.to_string()).collect();

                                HttpResponse::Ok().json( responses::RoleSelect {
                                    roles,
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
                        let (email, name) = (db_user.email.clone(), db_user.name.clone());

                        let sss_pub_token = match utils::issue_session_token(db_user, roles[0], user.remember_me, &redis_pool).await {
                            Ok(token) =>
                                token,
                            Err(e) if e.is::<types::error::Redis>() =>
                                return HttpResponse::InternalServerError().finish(),
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
                            if user.remember_me {
                                cookie.make_permanent();
                            }
                            cookie
                        };

                        HttpResponse::Ok()
                            .cookie(session_cookie)
                            .json(types::UserResponse {
                                email,
                                name,
                                client: None,
                                employee: None,
                                admin: None,
                            }
                        )
                    }
                }
                Err(e) => {
                    tracing::event!(target: "backend", tracing::Level::ERROR, "Wrong password: {:#?}", e);
                    HttpResponse::NotFound().json(responses::ErrorResponse {
                        error: USER_NOT_FOUND_MSG.to_string()
                    })
                }
            }
        }
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "User not found: {:#?}", e);
            HttpResponse::NotFound().json(responses::ErrorResponse {
                error: USER_NOT_FOUND_MSG.to_string()
            })
        }
    }
}

#[tracing::instrument(name = "Getting a user from DB.", skip(db, email),fields(user_email = %email))]
pub async fn get_user_who_is_active(
    db: &mongodb::Database,
    email: &String,
) -> Result<types::User> {
    let users_coll: Collection<User> = db.collection("user");
    let res = users_coll.find_one(
        doc! { "email": email, "isActive": true }
    ).await;

    match res {
        Ok(res) =>
            if let Some(user) = res {
                Ok(user)
            } else {
                tracing::event!(target: "mongodb", tracing::Level::ERROR, "User not found in DB. Email: {}", email);
                Err(anyhow!("User not found in DB."))
            },
        Err(e) => {
            tracing::event!(target: "mongodb", tracing::Level::ERROR, "Failed to query the mongodb database: {:#?}", e);
            Err(anyhow!(e))
        }
    }
}

#[tracing::instrument(
    name = "Logging a user in with role selection",
    skip(login, redis_pool),
    fields(
        role = %login.role,
        remember_me = %login.remember_me,
    )
)]
#[actix_web::post("/role-login")]
async fn role_login(
    login: web::Json<requests::RoleLoginUser>,
    redis_pool: web::Data<deadpool_redis::Pool>
) -> HttpResponse {
    tracing::info!(target: "backend", "Accessing LOGIN role selection.");

    let role = &login.role;
    let rolesel_pub_token = &login.rolesel_pub_token;

    if !vec!["client", "employee", "admin"].contains(&role.as_str()) {
        return HttpResponse::BadRequest().json(responses::ErrorResponse {
            error: format!("Unknown role: {}", role)
        })
    }

    let user = verify_roleselect_token(rolesel_pub_token.to_string(), &redis_pool).await;

    let role = match role.as_str() {
        "client" => "client",
        "employee" => "employee",
        "admin" => "admin",
        _ => unimplemented!()
    };

    match user {
        Ok(user) => {
            let session_cookie = {
                let sss_pub_token = match issue_session_token(user.clone(), role, login.remember_me, &redis_pool).await {
                    Ok(token) => token,
                    Err(e) => {
                        tracing::error!(target: "backend", "{}", e);
                        return HttpResponse::InternalServerError().finish();
                    }
                };

                let mut cookie = Cookie::build(SSS_COOKIE_NAME, sss_pub_token)
                    .path("/")
                    .http_only(true)
                    .finish();
                if login.remember_me {
                    cookie.make_permanent();
                }

                cookie
            };

            HttpResponse::Ok()
                .cookie(session_cookie)
                .json(responses::UserResponse {
                    email: user.email,
                    name: user.name,
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