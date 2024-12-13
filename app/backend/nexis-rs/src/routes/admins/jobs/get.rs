use crate::prelude::*;
use super::super::verify_session;
use types::mongodb::{ Job, IsCollection };

#[tracing::instrument(
    name = "Accessing admin's inventory upload endpoint",
    skip(db, redis_pool),
)]
#[actix_web::get("/jobs")]
pub async fn get_jobs(
    req: HttpRequest,
    db: web::Data<mongodb::Database>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResponse {
    let admin_session = match verify_session(&req, &db, &redis_pool).await {
        Ok(session) => session,
        Err(e) => {
            return e;
        }
    };

    let jobs_coll: Collection<Job> = db.collection(Job::coll_name());
    let mut jobs: Vec<Job> = Vec::new();

    match jobs_coll.find(doc! { "stores": { "$in": admin_session.stores }}).await {
        Ok(mut res) => {
            while let Ok(Some(job)) = res.try_next().await {
                jobs.push(job);
            }
        }
        Err(e) => {
            tracing::error!(target: "backend", "Error while getting jobs: {:#?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().json(jobs)
}