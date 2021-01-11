use actix_web::{web, error, HttpResponse, App, HttpServer, dev::ServiceRequest, dev::ServiceResponse};
use actix_session::CookieSession;
use actix_files::{Files, NamedFile};
use sqlx::postgres::{PgPoolOptions, PgPool};
use anyhow::Result;
use std::env;
use std::io::BufReader;
use std::fs::File;
use dotenv::dotenv;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use log::debug;
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};

mod routes;
mod models;

pub struct AppState {
    oauth: BasicClient,
    db_pool: PgPool,
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenv().ok();

    // oath2 setup
    let google_client_id = ClientId::new(
        env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID is not set in .env file")
    );
    let google_client_secret = ClientSecret::new(
        env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET is not set in .env file")
    );
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Invalid token endpoint URL");
    let redirect_url = RedirectUrl::new(
        env::var("GOOGLE_REDIRECT_URL").expect("GOOGLE_REDIRECT_URL is not set in .env file")
    ).expect("Invalid GOOGLE_REDIRECT_URL");
    let client = BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url),
    ).set_redirect_url(redirect_url);

    // db setup
    debug!("Creating DB pool");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await?;

    // front end setup
    let static_dir = env::var("STATIC_DIR").expect("STATIC_DIR is not set in .env file");

    // bind address setup
    let bind_addr = env::var("BIND_ADDR").expect("BIND_ADDR is not set in .env file");

    // ssl setup
    let cert_filename = env::var("CERT").expect("CERT is not set in .env file");
    let key_filename = env::var("KEY").expect("KEY is not set in .env file");
    let cert_file = File::open(&cert_filename).expect(&format!("Unable to open {}", &cert_filename));
    let key_file = File::open(&key_filename).expect(&format!("Unable to open {}", &key_filename));
    let cert_buf = &mut BufReader::new(cert_file);
    let key_buf = &mut BufReader::new(key_file);
    let cert_chain = certs(cert_buf).expect("Unable to create cert_chain");
    let mut keys = pkcs8_private_keys(key_buf).expect("Unable to create private keys");
    assert!(keys.len() > 0, "Unable to read key from file");
    let mut config = ServerConfig::new(NoClientAuth::new());  
    config.set_single_cert(cert_chain, keys.remove(0)).expect("Unable to set_single_cert");

    debug!("Starting web server");
    HttpServer::new(move || {
        let index = format!("{}/index.html", static_dir.clone());
        App::new()
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                error::InternalError::from_response("", HttpResponse::BadRequest().body(format!("{}", err))).into()
            }))
            .data(AppState { oauth: client.clone(), db_pool: db_pool.clone() })
            .wrap(CookieSession::private(&[0; 32]).secure(false))
            .configure(routes::init)
            .service(Files::new("/rob", &static_dir)
                .redirect_to_slash_directory()
                .index_file("index.html")
                .default_handler(move |req: ServiceRequest| {
                    let (http_req, _) = req.into_parts();

                    let tmp_index = index.clone();
                    async move {
                        let response = NamedFile::open(tmp_index)?
                            .into_response(&http_req)?;
                        Ok(ServiceResponse::new(http_req, response))
                    }
                }))
    })
    .bind_rustls(bind_addr, config)?
    .run()
    .await?;

    Ok(())
}
