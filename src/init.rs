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
    info!("sign: {}", config.auth.sign_key.len());
    info!("ctoken: {}", config.cloudflare.api_token.len());
    info!(
        "gtoken: {}",
        config.oauth.get(0).unwrap().client_secret.len()
    );
    config
}

#[cfg(feature = "ssr")]
pub fn cloudflare_config(config: &AppConfig) -> cloudflare_api::connect::ApiClientConfig {
    use cloudflare_api::connect::{ApiClientConfig, Credentials, HttpApiClient};
    ApiClientConfig {
        account_identifier: config.cloudflare.account_identifier.clone(),
        namespace_identifier: config.cloudflare.namespace_identifier.clone(),
        cloudflare_client: HttpApiClient::new(&Credentials::UserAuthToken {
            token: config.cloudflare.api_token.clone(),
        }),
    }
}

#[cfg(feature = "ssr")]
pub fn oauth2_client_init(config: &AppConfig) -> oauth2::basic::BasicClient {
    let google = config.oauth.get(0).unwrap();
    oauth2::basic::BasicClient::new(
        oauth2::ClientId::new(google.client_id.to_owned()),
        Some(oauth2::ClientSecret::new(google.client_secret.to_owned())),
        oauth2::AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
        Some(oauth2::TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap()),
    )
    .set_redirect_uri(oauth2::RedirectUrl::new(google.auth_landing_url.to_owned()).unwrap())
}

#[cfg(feature = "ssr")]
pub fn cors_layer() -> CorsLayer {
    // TODO: Get values from configuration

    let origins = [
        "http://localhost".parse().unwrap(),
        "http://127.0.0.1".parse().unwrap(),
        "http://0.0.0.0".parse().unwrap(),
        "https://hot-or-not-web-leptos-ssr-rupansh.fly.dev"
            .parse()
            .unwrap(),
        "https://hot-or-not-auth.fly.dev".parse().unwrap(),
        "https://hot-or-not-auth-stage.fly.dev".parse().unwrap(),
        "https://hot-or-not-web-leptos-ssr.fly.dev".parse().unwrap(),
    ];
    let cors_layer = CorsLayer::new()
        .allow_credentials(true)
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([ACCEPT, CONTENT_TYPE, ACCEPT_LANGUAGE, CONTENT_LANGUAGE]);
    cors_layer
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub auth: AuthConfig,
    pub cloudflare: CloudflareConfig,
    pub oauth: Vec<OAuthConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub cookie_domain: String,
    pub ic_url: String,
    pub sign_key: String,
    pub app_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CloudflareConfig {
    pub account_identifier: String,
    pub api_token: String,
    pub namespace_identifier: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OAuthConfig {
    pub provider_name: String,
    pub auth_landing_url: String,
    pub client_id: String,
    pub client_secret: String,
}
