pub mod storage_kv;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CloudflareResponse<T> {
    errors: Vec<Info>,
    messages: Vec<Info>,
    result: T,
    result_info: Option<String>,
    success: bool,
}

#[derive(Deserialize)]
struct Info {
    code: u32,
    message: String,
}
