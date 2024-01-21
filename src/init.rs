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
        .merge(Env::prefixed("AUTH_"))
        .merge(Env::prefixed("CLOUDFLARE_"))
        .extract()
        .unwrap();
    config
}

#[derive(Deserialize)]
pub struct AuthConfig {
    pub auth_ic_url: String,
    pub auth_sign_key: String,
    pub cloudflare_account_identifier: String,
    pub cloudflare_namespace_identifier: String,
    pub cloudflare_api_token: String,
}
