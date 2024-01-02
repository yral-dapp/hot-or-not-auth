extern crate tracing;

use tracing_subscriber::FmtSubscriber;

pub fn logging() {
    let auth_subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(auth_subscriber)
        .expect("setting tracing default failed");
}

pub fn configure() {}

pub struct AuthConfig {
    ic_url: String,
}
