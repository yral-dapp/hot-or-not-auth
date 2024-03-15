use once_cell::sync::Lazy;
use reqwest::Url;

pub static AUTH_DOMAIN: Lazy<Url> =
    Lazy::new(|| Url::parse("https://hot-or-not-auth-stage.fly.dev").unwrap());

pub static APP_DOMAIN: Lazy<Url> =
    Lazy::new(|| Url::parse("https://hot-or-not-web-leptos-ssr-rupansh.fly.dev").unwrap());
