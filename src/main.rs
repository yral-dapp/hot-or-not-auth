use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    mod auth;
    mod init;
    mod store;

mod handlers {
    use hot_or_not_auth::app::App;
    use crate::auth::identity::IdentityKeeper;
    use axum::{
        body::Body as AxumBody,
        extract::{Path, RawQuery, State},
        http::{header::HeaderMap, Request},
        response::{IntoResponse, Response},
    };
    use leptos::*;
    use leptos_axum::handle_server_fns_with_context;

    pub async fn server_fn_handler(
        State(app_state): State<IdentityKeeper>,
        path: Path<String>,
        headers: HeaderMap,
        raw_query: RawQuery,
        request: Request<AxumBody>,
    ) -> impl IntoResponse {
        handle_server_fns_with_context(
            path, headers, raw_query,
            move || {
                provide_context(app_state.oauth_map.clone());
            },
            request,
        )
        .await
    }

    pub async fn leptos_routes_handler(
        State(app_state): State<IdentityKeeper>,
        req: Request<AxumBody>,
    ) -> Response {
        let handler = leptos_axum::render_route_with_context(
            app_state.leptos_options.clone(),
            app_state.routes.clone(),
            move || {
                provide_context(app_state.oauth_map.clone());
            },
            App,
        );
        handler(req).await.into_response()
    }
}
}}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use auth::identity;
    use axum::{
        routing::{get, post},
        Router,
    };
    use axum_extra::extract::cookie::Key;
    use handlers::*;
    use hot_or_not_auth::{app::App, fileserve::file_and_error_handler};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use std::{collections::HashMap, sync::Arc};
    use tokio::sync::RwLock;
    use tower::ServiceBuilder;
    use tower_http::cors::CorsLayer;

    init::logging();
    let config = init::configure();

    let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let identity_keeper = identity::IdentityKeeper {
        leptos_options,
        oauth_map: Arc::new(RwLock::new(HashMap::new())),
        key: Key::from(config.auth_sign_key.as_bytes()),
        routes: routes.clone(),
    };
    let identity_keeper: identity::IdentityKeeper = identity_keeper;
    let service = ServiceBuilder::new().layer(CorsLayer::permissive());

    let app = Router::new()
        // .route("/", get(|| async { "Welcome to HotOrNot!" }))
        // .route("/generate_session", post(identity::generate_session))
        .route("/*fn_name", get(server_fn_handler).post(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .layer(service)
        .with_state(identity_keeper);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
