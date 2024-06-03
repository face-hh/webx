mod routes;

use axum::{
    error_handling::HandleErrorLayer,
    http::{StatusCode, Uri},
    routing::{delete, get, post, put},
    BoxError, Router,
};
use tokio::net::TcpListener;

use mongodb::Client;

use async_once::AsyncOnce;
use lazy_static::lazy_static;
use std::net::SocketAddr;
use std::sync::Arc;

use tower::{buffer::BufferLayer, ServiceBuilder};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

use routes::domain::{delete_domain, domain_list, domain_register, get_domain, update_domain};
use routes::tlds::tlds;

const TLD: [&str; 18] = [
    "mf", "btw", "fr", "yap", "dev", "scam", "zip", "root", "web", "rizz", "habibi", "sigma",
    "now", "it", "soy", "lol", "uwu", "ohio",
];

lazy_static! {
    static ref CLIENT: AsyncOnce<Client> = AsyncOnce::new(async {
        let uri = std::env::var("MONGO_URL").unwrap();
        let client = Client::with_uri_str(&uri).await.unwrap();
        tracing::info!("Created the mongo client!");

        client
    });
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (
        StatusCode::NOT_FOUND,
        format!("you entered a wrong path: {}", uri.path()),
    )
}

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(60 * 60)
            .burst_size(1)
            .finish()
            .unwrap(),
    );

    let app = Router::new()
        // add non-domain routes after route_layer -> https://docs.rs/axum/latest/axum/struct.Router.html#method.route_layer
        .route("/domain", post(domain_register))
        .route_layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled error: {}", err),
                    )
                }))
                .layer(BufferLayer::new(1024))
                .layer(GovernorLayer {
                    config: governor_conf,
                }),
        )
        .route("/domain/:name/:tld", get(get_domain))
        .route("/domain/:key", delete(delete_domain))
        .route("/domains", get(domain_list))
        .route("/domain/:key", put(update_domain))
        .route("/tlds", get(tlds))
        .fallback(fallback)
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled error: {}", err),
                    )
                }))
                .layer(BufferLayer::new(1024)),
        );

    // change later
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}!", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
