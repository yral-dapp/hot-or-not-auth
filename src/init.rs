extern crate tracing;

use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

pub fn logging() {
    tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .init();
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
