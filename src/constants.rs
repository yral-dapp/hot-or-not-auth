use once_cell::sync::Lazy;
use reqwest::Url;

pub static AUTH_DOMAIN: Lazy<Url> =
    Lazy::new(|| Url::parse("https://hot-or-not-auth-rupansh.fly.dev").unwrap());
