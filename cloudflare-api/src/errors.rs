use thiserror::Error;

#[derive(Error, Debug)]
pub enum CloudflareError {
    #[error("Failed to send reqwest: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("serde json failed to convert: {0}")]
    SerdeError(String),
    #[error("cloudflare call failed: {0}")]
    CloudflareError(String),
}
