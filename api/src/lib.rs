pub mod error;
pub mod filters;
pub mod handlers;
pub mod permission;
pub mod routes;

use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{cookie::Key, middleware, web, App, HttpServer};
use book_manager_service::sea_orm::{Database, DatabaseConnection};

use listenfd::ListenFd;
use migration::{Migrator, MigratorTrait};
use rustls::{Certificate, PrivateKey};
use serde::{Deserialize, Serialize};
use std::{env, fs::File, io::BufReader};
use tera::Tera;
use std::sync::RwLock;

use crate::routes::general_routes;

#[derive(Debug)]
pub struct AppState {
    templates: RwLock<Tera>,
    conn: DatabaseConnection,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct FlashData {
    kind: String,
    message: String,
}

impl FlashData {
    fn error<T: ToString>(message: T) -> Self {
        Self {
            kind: "error".to_string(),
            message: message.to_string(),
        }
    }
    fn success<T: ToString>(message: T) -> Self {
        Self {
            kind: "success".to_string(),
            message: message.to_string(),
        }
    }
}

fn flash_error<T: ToString>(session: &Session, msg: T) -> actix_web::Result<()> {
    session.insert("flash", FlashData::error(msg))?;
    Ok(())
}

fn flash_success<T: ToString>(session: &Session, msg: T) -> actix_web::Result<()> {
    session.insert("flash", FlashData::success(msg))?;
    Ok(())
}

fn get_secret_key() -> Key {
    Key::from(&[0; 64])
}

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    // get env vars
    dotenvy::dotenv().ok();
    
    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    // establish connection to database and apply migrations
    // -> create post table if not exists
    let conn = Database::connect(&db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    // load tera templates and build app state
    let template_dir = match env::var("TEMPLATES_DIR") {
        Ok(val) => format!("{}/{}", val, "**/*"),
        Err(_e) => concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*").to_string(),
    };

    println!("Loading templates from {template_dir}");

    let mut templates = Tera::new(&template_dir).unwrap();
    templates.register_filter("is_overdue", filters::is_overdue);
    // templates.register_filter("format_date", filters::format_date);

    // create server and try to serve over socket if possible
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        let state = AppState {
            templates: RwLock::new(templates.clone()),
            conn: conn.clone(),
        };

        App::new()
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                get_secret_key(),
            ))
            .app_data(web::Data::new(state))
            .wrap(middleware::Logger::default()) // enable logger
            .configure(general_routes)
    });

    // 配置 SSL
    let config = load_ssl_config();

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind_rustls_021(&server_url, config)?,
    };

    println!("Starting server at {server_url}");
    server.run().await?;

    Ok(())
}

fn load_ssl_config() -> rustls::ServerConfig {
    let certificate_dir =
        env::var("CERTIFICATE_DIR").expect("CERTIFICATE_DIR is not set in .env file");

    // 加载 SSL 证书
    let cert_file =
        &mut BufReader::new(File::open(format!("{}/certificate.crt", certificate_dir)).unwrap());
    let key_file = &mut BufReader::new(File::open(format!("{}/key.pem", certificate_dir)).unwrap());
    let cert = rustls_pemfile::certs(cert_file).unwrap();
    let cert_chain: Vec<_> = cert.into_iter().map(Certificate).collect();
    let key = rustls_pemfile::pkcs8_private_keys(key_file).unwrap()[0].clone();
    let key_der = PrivateKey(key);

    // 配置 SSL
    let config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key_der)
        .unwrap();

    config
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
