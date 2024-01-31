pub mod storage_kv;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CloudflareResponse<T> {
    pub errors: Vec<Info>,
    pub messages: Vec<Info>,
    pub result: Option<T>,
    pub result_info: Option<String>,
    pub success: bool,
}

#[derive(Debug, Deserialize)]
pub struct Info {
    pub code: u32,
    pub message: String,
}
