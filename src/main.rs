mod app;
mod auth;
mod constants;
mod error_template;
mod fileserve;
mod init;
mod metadata;
mod page;
mod providers;
mod store;

#[cfg(feature = "ssr")]
mod handlers {
    use crate::{app::App, auth::identity::AppState};
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
        State(app_state): State<AppState>,
        path: Path<String>,
        request: Request<AxumBody>,
    ) -> impl IntoResponse {
        info!("path: {}", path.as_str());
        handle_server_fns_with_context(
            move || {
                provide_context(app_state.clone());
            },
            request,
        )
        .await
    }

    pub async fn leptos_routes_handler(
        State(app_state): State<AppState>,
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
    use crate::{
        app::App,
        auth::identity,
        fileserve::file_and_error_handler,
        init,
        metadata::user::{get_user_canister, update_user_metadata},
    };
    use axum::{
        routing::{get, post},
        Router,
    };
    use axum_extra::extract::cookie::Key;
    use handlers::*;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use reqwest::Url;
    use std::time::Duration;
    use tower::ServiceBuilder;
    use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

    init::logging();
    let app_config = init::configure();
    let oauth2_client = init::oauth2_client_init(&app_config);
    let cloudflare_config = init::cloudflare_config(&app_config);

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app_state = identity::AppState {
        leptos_options,
        key: Key::from(app_config.auth_sign_key.as_bytes()),
        routes: routes.clone(),
        oauth2_client,
        reqwest_client: reqwest::Client::new(),
        auth_domain: Url::parse(&app_config.auth_domain).unwrap(),
        app_domain: Url::parse(&app_config.auth_app_domain).unwrap(),
        cloudflare_config,
    };
    let app_state: identity::AppState = app_state;

    let service = ServiceBuilder::new().layer(init::cors_layer());

    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .route("/rest_api/update_user_metadata", post(update_user_metadata))
        .route("/rest_api/get_user_canister", post(get_user_canister))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .layer(service)
        .layer((
            TraceLayer::new_for_http(),
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            TimeoutLayer::new(Duration::from_secs(10)),
        ))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}

#[cfg(feature = "ssr")]
async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
