// src/routes/health.rs
use crate::prelude::*;
use actix_web::cookie::SameSite;

#[tracing::instrument]
#[actix_web::get("/health-check")]
pub async fn health_check() -> actix_web::HttpResponse {
    tracing::event!(target: "backend", tracing::Level::DEBUG, "Accessing health-check endpoint.");

    let cookie = Cookie::build("mycookie", "somevalue")
        .path("/")
        .same_site(SameSite::None)
        .secure(true)
        .http_only(true)
        .finish();

    actix_web::HttpResponse::Ok()
        .cookie(cookie)
        .json("Application is safe and healthy. :)")
}