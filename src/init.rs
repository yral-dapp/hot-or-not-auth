use cfg_if::cfg_if;
use serde::Deserialize;

cfg_if! { if #[cfg(feature = "ssr")] {
extern crate tracing;

use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use http::{
    header::{ACCEPT, ACCEPT_LANGUAGE, CONTENT_LANGUAGE, CONTENT_TYPE},
    Method,
};
use tower_http::cors::CorsLayer;
}}

#[cfg(feature = "ssr")]
pub fn logging() {
    tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .init();
}

#[cfg(feature = "ssr")]
pub fn configure() -> AppConfig {
    use tracing::log::info;
    let config: AppConfig = Figment::new()
        .merge(Toml::file("AppConfig.toml"))
        .merge(Env::raw())
        .extract()
        .unwrap();
    info!("sign: {}", config.auth_sign_key.len());
    info!("ctoken: {}", config.cloudflare_api_token.len());
    info!("gtoken: {}", config.google_client_secret.len());
    config
}

#[cfg(feature = "ssr")]
pub fn cloudflare_config(config: &AppConfig) -> cloudflare_api::connect::ApiClientConfig {
    use cloudflare_api::connect::{ApiClientConfig, Credentials, HttpApiClient};
    ApiClientConfig {
        account_identifier: config.cloudflare_account_identifier.clone(),
        namespace_identifier: config.cloudflare_namespace_identifier.clone(),
        cloudflare_client: HttpApiClient::new(&Credentials::UserAuthToken {
            token: config.cloudflare_api_token.clone(),
        }),
    }
}

#[cfg(feature = "ssr")]
pub fn oauth2_client_init(config: &AppConfig) -> oauth2::basic::BasicClient {
    oauth2::basic::BasicClient::new(
        oauth2::ClientId::new(config.google_client_id.to_owned()),
        Some(oauth2::ClientSecret::new(
            config.google_client_secret.to_owned(),
        )),
        oauth2::AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
        Some(oauth2::TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap()),
    )
    .set_redirect_uri(oauth2::RedirectUrl::new(config.google_auth_landing_url.to_owned()).unwrap())
}

#[cfg(feature = "ssr")]
pub fn cors_layer(_env: leptos_config::Env) -> CorsLayer {
    CorsLayer::permissive() 
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub auth_ic_url: String,
    pub auth_sign_key: String,
    pub cookie_domain: String,
    pub auth_domain: String,
    pub app_domain: String,

    pub cloudflare_account_identifier: String,
    pub cloudflare_api_token: String,
    pub cloudflare_namespace_identifier: String,

    // TODO: Config controlled from dashboard
    pub google_auth_landing_url: String,
    pub google_client_id: String,
    pub google_client_secret: String,
}
