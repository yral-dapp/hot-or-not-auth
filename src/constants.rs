use once_cell::sync::Lazy;
use reqwest::Url;

pub static AUTH_URL: Lazy<Url> =
    Lazy::new(|| Url::parse("https://hot-or-not-auth-stage.fly.dev").unwrap());

pub static WEB_URL: Lazy<Url> =
    Lazy::new(|| Url::parse("https://hot-or-not-web-leptos-ssr-rupansh.fly.dev").unwrap());