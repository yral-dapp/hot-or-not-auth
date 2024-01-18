#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
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

    init::logging();
    let config = init::configure();

    let identity_keeper = identity::IdentityKeeper {
        oauth_map: Arc::new(RwLock::new(HashMap::new())),
        key: Key::from(config.auth_sign_key.as_bytes()),
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

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
