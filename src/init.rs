extern crate tracing;

use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;
use tracing_subscriber::FmtSubscriber;

pub fn logging() {
    let auth_subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(auth_subscriber)
        .expect("setting tracing default failed");
}

pub fn configure() -> AuthConfig {
    let config: AuthConfig = Figment::new()
        .merge(Toml::file("AuthConfig.toml"))
        .extract()
        .unwrap();
    config
}

#[derive(Deserialize)]
pub struct AuthConfig {
    pub ic_url: String,
    pub cloudflare_config: CloudflareConfig,
}

#[derive(Deserialize)]
pub struct CloudflareConfig {
    pub account_identifier: String,
    pub namespace_identifier: String,
    pub api_token: String,
}
