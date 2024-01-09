mod auth;
mod init;
mod providers;
mod store;

use auth::identity;
use axum::{
    routing::{get, post},
    Router,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    init::logging();
    let config = init::configure();

    let identity_keeper = identity::IdentityKeeper {
        oauth_map: HashMap::new(),
    };
    let identity_keeper: Arc<RwLock<identity::IdentityKeeper>> =
        Arc::new(RwLock::new(identity_keeper));
    let service = ServiceBuilder::new().layer(CorsLayer::permissive());
    let app = Router::new()
        .route("/", get(|| async { "Welcome to HotOrNot!" }))
        .route("/generate_session", post(identity::generate_session))
        .layer(service)
        .with_state(identity_keeper);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
