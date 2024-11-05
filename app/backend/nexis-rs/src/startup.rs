// src/startup.rs
use crate::prelude::*;
use actix_web::middleware;
use std::{ fs::File, io::BufReader };
use rustls::{ pki_types::PrivateKeyDer, ServerConfig };
use rustls_pemfile::{certs, pkcs8_private_keys};

pub struct Application {
    port: u16,
    server: actix_web::dev::Server,
}

impl Application {
    pub async fn build(
        settings: crate::settings::Settings,
        db: Option<mongodb::Database>,
    ) -> Result<Self, std::io::Error> {
        let db = if let Some(db) = db {
            db
        } else if settings.debug {
            get_mongodb_database(&settings.database).await.expect("")
        } else {
            let db_uri = std::env::var("DATABASE_URI").expect("Failed to get DATABASE_URI.");
            // TODO: Handle errors with match
            let options = ClientOptions::parse(db_uri).resolver_config(ResolverConfig::cloudflare()).await.expect("Failed to get client options");
            let client = mongodb::Client::with_options(options).expect("Failed to get database client");
            let db = client.database(&settings.database.database_name);
            db
        };

        let port = settings.application.port;
        let server = run(db, settings).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub async fn get_mongodb_database(
    settings: &crate::settings::DatabaseSettings,
) -> Result<mongodb::Database, mongodb::error::Error> {
    let options = ClientOptions::parse(&settings.uri).resolver_config(ResolverConfig::cloudflare()).await?;
    let client = mongodb::Client::with_options(options)?;
    let db = client.database(&settings.database_name);

    Ok(db)
}

async fn run(
    db: mongodb::Database,
    settings: crate::settings::Settings,
) -> Result<actix_web::dev::Server, std::io::Error> {
    // Database connection application state
    let db = actix_web::web::Data::new(db);

    // Redis connection pool
    let cfg = deadpool_redis::Config::from_url(settings.clone().redis.uri);
    let redis_pool = cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Cannot create deadpool redis.");
    let redis_pool_data = actix_web::web::Data::new(redis_pool);

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(
            actix_cors::Cors::default()
                .allowed_origin(&settings.frontend_url)
                .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE", "OPTIONS"])
                .allowed_headers(vec![
                    actix_web::http::header::AUTHORIZATION,
                    actix_web::http::header::ACCEPT,
                ])
                .allowed_header(actix_web::http::header::CONTENT_TYPE)
                .expose_headers(&[actix_web::http::header::CONTENT_DISPOSITION])
                .supports_credentials()
                .max_age(3600),
            )
            .service(crate::routes::health_check)
            .service(crate::routes::search_suggestions)
            .service(crate::routes::search_items)
            .service(crate::routes::search_item_details)
            .configure(crate::routes::debug_routes_config)
            .configure(crate::routes::auth_routes_config)
            .configure(crate::routes::client_routes_config)
            // Add database pool to application state
            .app_data(db.clone())
            // Add redis pool to application state
            .app_data(redis_pool_data.clone())
            .wrap(middleware::NormalizePath::trim())
    });

    let server = if settings.application.protocol == "https" {
        let rustls_config = load_rustls_config();
        server.bind_rustls_0_23(format!("{}:{}", settings.application.host, settings.application.port), rustls_config)?
        .run()
    } else {
        let address = format!(
            "{}:{}",
            settings.application.host, settings.application.port
        );
        let listener = std::net::TcpListener::bind(&address)?;
        server.listen(listener)?
        .run()
    };

    Ok(server)
}

fn load_rustls_config() -> rustls::ServerConfig {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    // init server config builder with safe defaults
    let config = ServerConfig::builder().with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("cert/cert.pem").expect("cannot find `cert/cert.pem` file"));
    let key_file = &mut BufReader::new(File::open("cert/key.pem").expect("cannot find `cert/key.pem` file"));

    // convert files to key/cert objects
    let cert_chain = certs(cert_file).collect::<Result<Vec<_>, _>>().unwrap();
    let mut keys = pkcs8_private_keys(key_file)
        .map(|key| key.map(PrivateKeyDer::Pkcs8))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}