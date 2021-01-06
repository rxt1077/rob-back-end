use actix_web::{web, get, put, post, delete, HttpResponse, Responder};
use actix_session::Session;
use log::debug;

use crate::models::{Test, User};
use crate::AppState;
use crate::{auth_instructor, auth_signed_in};

// creates a test
#[post("/test")]
async fn create_test(test: web::Json<Test>, session: Session,
                     data: web::Data<AppState>) -> impl Responder {
    auth_instructor!(session);
    match Test::create(test.into_inner(), &data.db_pool).await {
        Ok(test) => HttpResponse::Ok().json(test),
        Err(e) => {
            debug!("Error trying to create test: {:?}", e);
            HttpResponse::BadRequest().body("Error trying to create test")
        }
    }
}

// gets a single test
#[get("/test/{id}")]
async fn get_test(id: web::Path<i32>, session: Session, data: web::Data<AppState>) -> impl Responder {
    auth_instructor!(session);
    match Test::find(id.into_inner(), &data.db_pool).await {
        Ok(test) => HttpResponse::Ok().json(test),
        Err(e) => {
            debug!("Error trying to get test: {:?}", e);
            HttpResponse::BadRequest().body("Error trying to get test")
        }
    }
}

// gets all tests
#[get("/test")]
async fn list_tests(session: Session, data: web::Data<AppState>)
    -> impl Responder {
    auth_instructor!(session);
    match Test::list(&data.db_pool).await {
        Ok(tests) => HttpResponse::Ok().json(tests),
        Err(e) => {
            debug!("Error trying to list tests: {:?}", e);
            HttpResponse::BadRequest().body("Error trying to list tests")
        }
    }
}

// deletes a test
#[delete("/test/{id}")]
async fn delete_test(id: web::Path<i32>, session: Session,
                     data: web::Data<AppState>) -> impl Responder {
    auth_instructor!(session);
    match Test::delete(id.into_inner(), &data.db_pool).await {
        Ok(test) => HttpResponse::Ok().json(test),
        Err(e) => {
            debug!("Error trying to delete test: {:?}", e);
            HttpResponse::BadRequest().body("Error trying to delete test")
        }
    }
}

// updates a test
#[put("/test/{id}")]
async fn update_test(id: web::Path<i32>, test: web::Json<Test>,
                     session: Session, data: web::Data<AppState>)
    -> impl Responder {
    auth_instructor!(session);
    let test = test.into_inner();
    if id.into_inner() != test.id {
        return HttpResponse::BadRequest().body("Error id in path and in json data must match")
    }
    match Test::update(test, &data.db_pool).await {
        Ok(test) => HttpResponse::Ok().json(test),
        Err(e) => {
            debug!("Error trying to update test: {:?}", e);
            HttpResponse::BadRequest().body("Error trying to update test")
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create_test);
    cfg.service(get_test);
    cfg.service(list_tests);
    cfg.service(delete_test);
    cfg.service(update_test);
}
