mod auth;
mod init;
mod store;

use auth::identity;
use axum::{
    routing::{get, post},
    Router,
};
use axum_extra::extract::cookie::Key;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    init::logging();
    // let config = init::configure();

    let identity_keeper = identity::IdentityKeeper {
        oauth_map: Arc::new(RwLock::new(HashMap::new())),
        // Generate a secure key
        //
        // You probably don't wanna generate a new one each time the app starts though
        key: Key::generate(),
    };
    let identity_keeper: identity::IdentityKeeper = identity_keeper;
    let service = ServiceBuilder::new().layer(CorsLayer::permissive());
    let app = Router::new()
        .route("/", get(|| async { "Welcome to HotOrNot!" }))
        .route("/generate_session", post(identity::generate_session))
        .layer(service)
        .with_state(identity_keeper);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
