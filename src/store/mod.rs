cfg_if::cfg_if! {
if #[cfg(feature = "ssr")] {
mod cloudflare;
pub mod mock;

use enum_dispatch::enum_dispatch;
use std::collections::HashMap;
use cloudflare_api::connect::ApiClientConfig;
use mock::MockKV;

#[enum_dispatch]
pub(crate) trait KVStore: Send {
    async fn read_kv(&self, key_name: &str) -> Option<String>;
    async fn read_metadata(&self, key_name: &str) -> Option<HashMap<String, String>>;
    async fn write_kv(&self, key_name: &str, value: &str, metadata: HashMap<&str, &str>) -> Option<String>;
    async fn delete_kv(&self, key_name: &str) -> Option<String>;
}

#[derive(Clone)]
#[enum_dispatch(KVStore)]
pub enum KVStoreProv {
    Mock(MockKV),
    Cf(ApiClientConfig)
}

}}
