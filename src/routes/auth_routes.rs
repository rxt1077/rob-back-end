use actix_session::Session;
use actix_web::{get, web, Responder, HttpResponse};
use oauth2::{ AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope, TokenResponse};
use actix_web::http::header;
use serde::Deserialize;
use oauth2::reqwest::http_client;
use actix_web::client::Client;

use crate::AppState;
use crate::models::User;

// macros to make it easier to protect routes

// returns early if a user is not signed in
#[macro_export]
macro_rules! auth_signed_in {
    ($session:expr) => {
        match $session.get::<User>("user") {
            Ok(Some(user)) => user,
            _ => return HttpResponse::Unauthorized().body("Must be signed in"),
        }
    }
}

// returns early if a user is not an instructor
#[macro_export]
macro_rules! auth_instructor {
    ($session:expr) => {
        match auth_signed_in!($session) {
            user if user.instructor => user,
            _ => return HttpResponse::Unauthorized().body("Must be an instructor"),
        }
    }
}

// source: https://github.com/pka/actix-web-oauth2/blob/master/src/bin/google.rs

#[get("/login")]
async fn login(session: Session, data: web::Data<AppState>) -> impl Responder {
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    session.set("pkce_code_verifier", pkce_code_verifier).unwrap();

    let (authorize_url, csrf_state) = &data
        .oauth
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()))
        .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()))
        .set_pkce_challenge(pkce_code_challenge)
        .url();
    session.set("csrf_state", csrf_state).unwrap();


    HttpResponse::Found()
        .header(header::LOCATION, authorize_url.to_string())
        .finish()
}

#[derive(Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
    scope: String,
}

#[derive(Deserialize)]
struct GoogleUserProfile {
    id: String,
    email: String,
    verified_email: bool,
    name: String,
    given_name: String,
    family_name: String,
    picture: String,
    locale: String,
    hd: String,
}

#[get("/auth")]
async fn auth(session: Session, data: web::Data<AppState>, params: web::Query<AuthRequest>) -> impl Responder {
    // pull needed variables out of the session cookie
    let pkce_code_verifier = match session.get::<PkceCodeVerifier>("pkce_code_verifier") {
        Ok(Some(pkce_code_verifier)) => pkce_code_verifier,
        _ => return HttpResponse::BadRequest().body("Couldn't get pkce_code_verifier from session")
    };
    let csrf_state = match session.get::<CsrfToken>("csrf_state") {
        Ok(Some(csrf_state)) => csrf_state,
        _ => return HttpResponse::BadRequest().body("Couldn't get csrf_state from session")
    };

    // pull response parameters out of URL
    let code = AuthorizationCode::new(params.code.clone());
    let state = CsrfToken::new(params.state.clone());

    // guard against cross-site request forgery
    if state.secret() != csrf_state.secret() {
        return HttpResponse::BadRequest().body("state parameter and csrf_state do not match")
    }

    let _scope = params.scope.clone();

    // reach out to Google to obtain an oauth token
    let token = match &data.oauth.exchange_code(code).set_pkce_verifier(pkce_code_verifier).request(http_client) {
        Ok(token) => token.clone(),
        _ => return HttpResponse::BadRequest().body("Unable to obtain token")
    };

    // get user information from Google API
    let client = Client::default();
    let response = client.get("https://www.googleapis.com/oauth2/v1/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await;
    let mut response = match response {
        Ok(response) => response,
        _ => return HttpResponse::BadRequest().body("Unable to retrieve userinfo from Google API")
    };
    let user_profile = match response.json::<GoogleUserProfile>().await {
        Ok(user_profile) => user_profile,
        _ => return HttpResponse::BadRequest().body("Unable to deserialize userinfo from Google API")
    };
    let ucid = match user_profile.email.split('@').next() {
        Some(ucid) => ucid.to_lowercase(),
        _ => return HttpResponse::BadRequest().body("Unable to determine UCID from email address")
    };

    // build a default user from the Google profile
    let mut user = User {
        ucid: ucid.clone(),
        instructor: false,
        google_id: user_profile.id,
        google_email: user_profile.email,
        google_verified_email: user_profile.verified_email,
        google_name: user_profile.name,
        google_given_name: user_profile.given_name,
        google_family_name: user_profile.family_name,
        google_picture: user_profile.picture,
        google_locale: user_profile.locale,
        google_hd: user_profile.hd,
    };

    // try to lookup the user in the db
    match User::find(ucid, &data.db_pool).await {
        // existing user: retrieve the non-google attributes (instructor) from db
        // save whole user in case attributes retrieved from Google have changed
        Ok(db_user) => {
            user.instructor = db_user.instructor;
            user = match User::update(user, &data.db_pool).await {
                Ok(user) => user,
                _ => return HttpResponse::BadRequest().body("Unable to update user")
            }
        }
        // new user: save defaults and Google attributes to db
        _ => {
            user = match User::create(user, &data.db_pool).await {
                Ok(user) => user,
                _ => return HttpResponse::BadRequest().body("Unable to create user")
            }
        }
    }

    // put user information in session
    session.set("user", user).unwrap();

    HttpResponse::Found()
        .header(header::LOCATION, "/rob/index.html")
        .finish()
}

#[get("/test_instructor")]
async fn test_instructor(session: Session, data: web::Data<AppState>) -> impl Responder {
    match User::find("test_instructor".to_string(), &data.db_pool).await {
        Ok(user) => {
            session.set("user", user).unwrap();
        }
        _ => {
            return HttpResponse::BadRequest().body("Could not find test_instructor in DB")
        }
    }

    HttpResponse::Found()
        .header(header::LOCATION, "/rob/index.html")
        .finish()
}

#[get("/test_student")]
async fn test_student(session: Session, data: web::Data<AppState>) -> impl Responder {
    match User::find("test_student".to_string(), &data.db_pool).await {
        Ok(user) => {
            session.set("user", user).unwrap();
        }
        _ => {
            return HttpResponse::BadRequest().body("Could not find test_instructor in DB")
        }
    }

    HttpResponse::Found()
        .header(header::LOCATION, "/rob/index.html")
        .finish()
}

#[get("/session")]
async fn get_session(session: Session) -> impl Responder {
    match session.get::<User>("user") {
        Ok(Some(user)) => return HttpResponse::Ok().json(user),
        _ => return HttpResponse::BadRequest().body("No user in session")
    };
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(auth);
    cfg.service(get_session);
    cfg.service(test_instructor);
    cfg.service(test_student);
}
