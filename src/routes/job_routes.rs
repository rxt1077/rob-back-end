use actix_web::{web, get, post, delete, HttpResponse, Responder};
use actix_session::Session;

use crate::models::{Job, User};
use crate::AppState;
use crate::{auth_instructor, auth_signed_in};

// creates a job
#[post("/job")]
async fn create_job(job: web::Json<Job>, session: Session, data: web::Data<AppState>) -> impl Responder {
    auth_instructor!(session);
    match Job::create(job.into_inner(), &data.db_pool).await {
        Ok(job) => HttpResponse::Ok().json(job),
        _ => HttpResponse::BadRequest().body("Error trying to create job")
    }
}

// gets a single job
#[get("/job/{id}")]
async fn get_job(id: web::Path<i32>, session: Session, data: web::Data<AppState>) -> impl Responder {
    auth_instructor!(session);
    match Job::find(id.into_inner(), &data.db_pool).await {
        Ok(job) => HttpResponse::Ok().json(job),
        _ => HttpResponse::BadRequest().body("Error trying to get job")
    }
}

// gets all jobs
#[get("/job")]
async fn list_jobs(session: Session, data: web::Data<AppState>) -> impl Responder {
    auth_instructor!(session);
    match Job::list(&data.db_pool).await {
        Ok(jobs) => HttpResponse::Ok().json(jobs),
        _ => HttpResponse::BadRequest().body("Error trying to list jobs")
    }
}

// deletes a job
#[delete("/job/{id}")]
async fn delete_job(id: web::Path<i32>, session: Session, data: web::Data<AppState>) -> impl Responder {
    auth_instructor!(session);
    match Job::delete(id.into_inner(), &data.db_pool).await {
        Ok(job) => HttpResponse::Ok().json(job),
        _ => HttpResponse::BadRequest().body("Error trying to delete job")
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_job);
    cfg.service(get_job);
    cfg.service(list_jobs);
    cfg.service(delete_job);
}
