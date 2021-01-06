use actix_web::{web, post, put, get, delete, HttpResponse, Responder};
use actix_session::Session;

use crate::models::{Group, User};
use crate::AppState;
use crate::{auth_instructor, auth_signed_in};

#[post("/group")]
async fn create_group(group: web::Json<Group>, session: Session, data: web::Data<AppState>) -> impl Responder {
    auth_instructor!(session);
    match Group::create(group.into_inner(), &data.db_pool).await {
        Ok(group) => HttpResponse::Ok().json(group),
        _ => HttpResponse::BadRequest().body("Error trying to create group")
    }
}

#[get("/group")]
async fn list_groups(session: Session, data: web::Data<AppState>) -> impl Responder {
    auth_instructor!(session);
    match Group::list(&data.db_pool).await {
        Ok(groups) => HttpResponse::Ok().json(groups),
        _ => HttpResponse::BadRequest().body("Error trying to list groups")
    }
}

#[put("/group/{id}")]
async fn update_group(id: web::Path<i32>, group: web::Json<Group>, session: Session, data: web::Data<AppState>) -> impl Responder {
    auth_instructor!(session);
    let group = group.into_inner();
    if id.into_inner() != group.id {
        return HttpResponse::BadRequest().body("Error id in path and in json data must match")
    }
    match Group::update(group, &data.db_pool).await {
        Ok(group) => HttpResponse::Ok().json(group),
        _ => HttpResponse::BadRequest().body("Error trying to update group")
    }
}

#[get("/group/{id}")]
async fn get_group(id: web::Path<i32>, session: Session, data: web::Data<AppState>) -> impl Responder {
    auth_instructor!(session);
    match Group::find(id.into_inner(), &data.db_pool).await {
        Ok(group) => HttpResponse::Ok().json(group),
        _ => HttpResponse::BadRequest().body("Error trying to get group")
    }
}

#[delete("/group/{id}")]
async fn delete_group(id: web::Path<i32>, session: Session, data: web::Data<AppState>) -> impl Responder {
    auth_instructor!(session);
    match Group::delete(id.into_inner(), &data.db_pool).await {
        Ok(group) => HttpResponse::Ok().json(group),
        _ => HttpResponse::BadRequest().body("Error trying to delete group")
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_group);
    cfg.service(list_groups);
    cfg.service(update_group);
    cfg.service(get_group);
    cfg.service(delete_group);
}
