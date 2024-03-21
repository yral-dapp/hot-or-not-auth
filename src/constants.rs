use once_cell::sync::Lazy;
use reqwest::Url;

pub static AUTH_DOMAIN: Lazy<Url> = Lazy::new(|| Url::parse("https://auth.yral.com").unwrap());

pub static APP_DOMAIN: Lazy<Url> = Lazy::new(|| Url::parse("https://yral.com").unwrap());
