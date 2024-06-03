mod models;
mod routes;

use crate::config::{self, Config};
use actix_web::{web, web::Data, App, HttpServer};
use lazy_static::lazy_static;
use models::Domain;
use mongodb::{bson::doc, options::ClientOptions, Client, Collection};
use rate_limit::backend::{memory::InMemoryBackend, SimpleInputFunctionBuilder};
use std::time::Duration;
use tokio::sync::Mutex as TokioMutex;

lazy_static! {
    static ref DB: TokioMutex<Option<Collection<Domain>>> = TokioMutex::new(None);
}

async fn connect_to_mongo(config: &Config) {
    let mut client_options = ClientOptions::parse(&config.server.mongo.connection).await.unwrap();
    client_options.app_name = Some(config.server.mongo.app_name.clone());

    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&config.server.mongo.db_name);
    let collection = db.collection::<Domain>("domains");

    let mut db_lock = DB.lock().await;
    *db_lock = Some(collection);

    log::info!("MongoDB server connected");
}

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    let config = config::read();
    let backend = InMemoryBackend::builder().build();

    connect_to_mongo(&config).await;

    // generate api keys and store in kv db, be more leinent with ratelimit on those users

    /* cli:
     server <start/generate-key/--config (config.toml:default)>
    */

    let app = move || {
        let config = config::read();
        let input = SimpleInputFunctionBuilder::new(Duration::from_secs(60), 10).real_ip_key().build();
        let middleware = rate_limit::RateLimiter::builder(backend.clone(), input).add_headers().build();

        App::new()
            .app_data(Data::new(config))
            .service(routes::index)
            .service(routes::get_domain)
            .service(routes::update_domain)
            .service(routes::delete_domain)
            .service(routes::get_domains)
            .service(routes::get_tlds)
            .route("/domain", web::post().wrap(middleware).to(routes::create_domain))
    };

    log::info!("Listening on {}", config.get_address());
    HttpServer::new(app).bind(config.get_address())?.run().await
}
