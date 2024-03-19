use cloudflare_api::{
    connect::ApiClientConfig,
    endpoints::storage_kv::{DeleteKV, ReadKV, ReadMetadata, WriteKVWithMetadata},
};
use std::collections::HashMap;
use tracing::log::{error, info, warn};

use super::KVStore;

impl KVStore for ApiClientConfig {
    async fn read_kv(&self, key_name: &str) -> Option<String> {
        let end_point = ReadKV {
            account_identifier: &self.account_identifier,
            namespace_identifier: &self.namespace_identifier,
            key_name,
        };
        match self.cloudflare_client.send(end_point).await {
            Ok(response) => Some(response),
            Err(error) => {
                warn!("Error read_kv: {}", error);
                None
            }
        }
    }

    async fn read_metadata(&self, key_name: &str) -> Option<HashMap<String, String>> {
        let end_point = ReadMetadata {
            account_identifier: &self.account_identifier,
            namespace_identifier: &self.namespace_identifier,
            key_name,
        };
        match self.cloudflare_client.send(end_point).await {
            Ok(response) => match response.success {
                true => Some(response.result.unwrap()),
                false => {
                    warn!("Error read_metadata: ");
                    for error in response.errors {
                        warn!("code: {}, message: {}", error.code, error.message);
                    }
                    None
                }
            },
            Err(error) => {
                warn!("Error read_metadata: {}", error);
                None
            }
        }
    }

    async fn write_kv(&self, key_name: &str, value: &str, metadata: HashMap<&str, &str>) -> Option<String> {
        let end_point = WriteKVWithMetadata {
            account_identifier: &self.account_identifier,
            namespace_identifier: &self.namespace_identifier,
            key_name,
            value: &format!(r#""{}""#, value),
            metadata,
        };
        let result = self.cloudflare_client.send(end_point).await;
        match result {
            Ok(result) => match result.success {
                true => {
                    info!("write kv success");
                    Some(result.success.to_string())
                }
                false => {
                    warn!("write kv failed: {:?}", result.errors);
                    None
                }
            },
            Err(error) => {
                error!("write kv error: {}", error);
                None
            }
        }
    }

    async fn delete_kv(&self, key_name: &str) -> Option<String> {
        let end_point = DeleteKV {
            account_identifier: &self.account_identifier,
            namespace_identifier: &self.namespace_identifier,
            key_name,
        };
        match self.cloudflare_client.send(end_point).await {
            Ok(result) => {
                info!("delete: {:?}", result);
                result.result
            }
            Err(error) => {
                error!("delete error: {}", error);
                None
            }
        }
    }
}
