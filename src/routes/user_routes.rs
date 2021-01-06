use actix_web::{get, web, HttpResponse, Responder};
use actix_session::Session;

use crate::models::User;
use crate::AppState;
use crate::{auth_instructor, auth_signed_in};

#[get("/user/{ucid}")]
async fn get_user(ucid: web::Path<String>, session: Session, data: web::Data<AppState>) -> impl Responder {
    auth_instructor!(session);
    let result = User::find(ucid.into_inner().to_lowercase(), &data.db_pool).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::BadRequest().body("User not found")
    }
}

#[get("/user")]
async fn list_users(session: Session, data: web::Data<AppState>) -> impl Responder {
    auth_instructor!(session);
    match User::list(&data.db_pool).await {
        Ok(users) => HttpResponse::Ok().json(users),
        _ => HttpResponse::BadRequest().body("Error trying to list users")
    }
}

/* SECURITY: people should only be able to create a user that matches their
 * authenticated email. 
#[put("/user/{ucid}")]
async fn update_user(ucid: web::Path<String>, user: web::Json<UserRequest>, data: web::Data<AppState>) -> impl Responder {
    let result = User::update(ucid.into_inner().to_lowercase(), user.into_inner(), &data.db_pool).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        _ => HttpResponse::BadRequest().body("Error trying to update user")
    }
}*/

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user);
    cfg.service(list_users);
//    cfg.service(update_user);
}
