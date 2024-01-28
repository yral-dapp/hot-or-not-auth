mod app;
mod auth;
mod error_template;
mod fileserve;
mod init;
mod page;
mod providers;
// mod store;

#[cfg(feature = "ssr")]
mod handlers {
    use crate::{app::App, auth::identity::IdentityKeeper};
    use axum::{
        body::Body as AxumBody,
        extract::{Path, State},
        http::Request,
        response::{IntoResponse, Response},
    };
    use leptos::*;
    use leptos_axum::handle_server_fns_with_context;
    use tracing::log::info;

    pub async fn server_fn_handler(
        State(app_state): State<IdentityKeeper>,
        path: Path<String>,
        request: Request<AxumBody>,
    ) -> impl IntoResponse {
        handle_server_fns_with_context(
            move || {
                provide_context(app_state.clone());
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
                provide_context(app_state.clone());
            },
            App,
        );
        handler(req).await.into_response()
    }
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use crate::{app::App, auth::identity, fileserve::file_and_error_handler, init};
    use axum::{routing::get, Router};
    use axum_extra::extract::cookie::Key;
    use handlers::*;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use std::{collections::HashMap, sync::Arc};
    use tokio::sync::RwLock;
    use tower::ServiceBuilder;
    use tower_http::cors::CorsLayer;

    init::logging();
    let auth_config = init::configure();
    let oauth2_client = init::oauth2_client_init(&auth_config);

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let identity_keeper = identity::IdentityKeeper {
        leptos_options,
        oauth_map: Arc::new(RwLock::new(HashMap::new())),
        key: Key::from(auth_config.auth_sign_key.as_bytes()),
        routes: routes.clone(),
        oauth2_client,
        reqwest_client: reqwest::Client::new(),
    };
    let identity_keeper: identity::IdentityKeeper = identity_keeper;
    let service = ServiceBuilder::new().layer(CorsLayer::permissive());

    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .layer(service)
        .with_state(identity_keeper);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
