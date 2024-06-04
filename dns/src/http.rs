mod helpers;
mod models;
mod ratelimit;
mod routes;

use crate::config::Config;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{http::Method, web, web::Data, App, HttpRequest, HttpServer};
use anyhow::{anyhow, Error};
use colored::Colorize;
use macros_rs::fmt::{crashln, string};
use ratelimit::RealIpKeyExtractor;
use std::{net::IpAddr, str::FromStr, time::Duration};

pub(crate) use models::Domain;

#[derive(Clone)]
pub(crate) struct AppState {
    trusted: IpAddr,
    config: Config,
    db: mongodb::Collection<Domain>,
}

pub fn get_token<'a>(req: &'a HttpRequest) -> Result<(&'a str, &'a str), Error> {
    let header = match req.headers().get("authorization") {
        Some(res) => res.to_str().unwrap_or(""),
        None => return Err(anyhow!("Missing header authorization")),
    };

    let chunks: Vec<&'a str> = header.split(":").collect();

    if chunks.len() == 2 {
        Ok((chunks[0], chunks[1]))
    } else {
        Err(anyhow!("Header '{}' does not contain exactly one colon", header))
    }
}

#[actix_web::main]
pub async fn start(cli: crate::Cli) -> std::io::Result<()> {
    let config = Config::new().set_path(&cli.config).read();

    let trusted_ip = match IpAddr::from_str(&config.server.address) {
        Ok(addr) => addr,
        Err(err) => crashln!("Cannot parse address.\n{}", string!(err).white()),
    };

    let governor_builder = GovernorConfigBuilder::default()
        .methods(vec![Method::POST])
        .period(Duration::from_secs(600))
        .burst_size(5)
        .key_extractor(RealIpKeyExtractor)
        .finish()
        .unwrap();

    let db = match config.connect_to_mongo().await {
        Ok(client) => client,
        Err(err) => crashln!("Failed to connect to MongoDB.\n{}", string!(err).white()),
    };

    let app = move || {
        let data = AppState {
            db: db.clone(),
            trusted: trusted_ip,
            config: Config::new().set_path(&cli.config).read(),
        };

        App::new()
            .app_data(Data::new(data))
            .service(routes::index)
            .service(routes::get_domain)
            .service(routes::update_domain)
            .service(routes::delete_domain)
            .service(routes::get_domains)
            .service(routes::get_tlds)
            .service(routes::check_domain)
            .service(routes::elevated_domain)
            .route("/domain", web::post().to(routes::create_domain).wrap(Governor::new(&governor_builder)))
    };

    log::info!("Listening on {}", config.get_address());
    HttpServer::new(app).bind(config.get_address())?.run().await
}
