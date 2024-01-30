use cloudflare_api::{
    connect::ApiClientConfig,
    endpoints::storage_kv::{DeleteKV, ReadKV, ReadMetadata, WriteKVWithMetadata},
};
use std::collections::HashMap;
use tracing::log::info;

pub async fn read_kv(key_name: &str, cloudflare_config: ApiClientConfig) -> Option<String> {
    let end_point = ReadKV {
        account_identifier: &cloudflare_config.account_identifier,
        namespace_identifier: &cloudflare_config.namespace_identifier,
        key_name,
    };
    match cloudflare_config.cloudflare_client.send(end_point).await {
        Ok(val) => Some(val),
        Err(error) => {
            info!("Error read_kv: {}", error);
            None
        }
    }
}

pub async fn read_metadata(
    key_name: &str,
    cloudflare_config: ApiClientConfig,
) -> Option<HashMap<String, String>> {
    let end_point = ReadMetadata {
        account_identifier: &cloudflare_config.account_identifier,
        namespace_identifier: &cloudflare_config.namespace_identifier,
        key_name,
    };
    match cloudflare_config.cloudflare_client.send(end_point).await {
        Ok(response) => {
            if response.success == true {
                Some(response.result)
            } else {
                info!("Error read_metadata: ");
                for error in response.errors {
                    info!("code: {}, message: {}", error.code, error.message);
                }
                None
            }
        }
        Err(error) => {
            info!("Error read_metadata: {}", error);
            None
        }
    }
}

pub async fn write_kv(
    key_name: &str,
    value: &str,
    metadata: HashMap<&str, &str>,
    cloudflare_config: ApiClientConfig,
) -> Option<String> {
    let end_point = WriteKVWithMetadata {
        account_identifier: &cloudflare_config.account_identifier,
        namespace_identifier: &cloudflare_config.namespace_identifier,
        key_name,
        value,
        metadata,
    };
    let result = cloudflare_config.cloudflare_client.send(end_point).await;
    None
}

pub async fn delete_kv(key_name: &str, cloudflare_config: ApiClientConfig) -> Option<String> {
    let end_point = DeleteKV {
        account_identifier: &cloudflare_config.account_identifier,
        namespace_identifier: &cloudflare_config.namespace_identifier,
        key_name,
    };
    let result = cloudflare_config.cloudflare_client.send(end_point).await;
    None
}
