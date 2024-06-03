mod models;
mod routes;

use crate::config::Config;
use actix_web::{web, web::Data, App, HttpServer};
use rate_limit::backend::{memory::InMemoryBackend, SimpleInputFunctionBuilder};
use std::time::Duration;

pub use models::Domain;

#[actix_web::main]
pub async fn start(cli: crate::Cli) -> std::io::Result<()> {
    let config = Config::new().set_path(&cli.config).read();
    let backend = InMemoryBackend::builder().build();

    config.connect_to_mongo(&crate::DB).await;

    let app = move || {
        let config = Config::new().set_path(&cli.config).read();
        let input = SimpleInputFunctionBuilder::new(Duration::from_secs(360), 5).real_ip_key().build();
        let middleware = rate_limit::RateLimiter::builder(backend.clone(), input).add_headers().build();

        App::new()
            .app_data(Data::new(config))
            .service(routes::index)
            .service(routes::get_domain)
            .service(routes::update_domain)
            .service(routes::delete_domain)
            .service(routes::get_domains)
            .service(routes::get_tlds)
            .service(routes::elevated_domain)
            .route("/domain", web::post().wrap(middleware).to(routes::create_domain))
    };

    log::info!("Listening on {}", config.get_address());
    HttpServer::new(app).bind(config.get_address())?.run().await
}
