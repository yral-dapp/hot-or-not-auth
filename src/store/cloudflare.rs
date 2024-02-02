use cloudflare_api::{
    connect::ApiClientConfig,
    endpoints::storage_kv::{DeleteKV, ReadKV, ReadMetadata, WriteKVWithMetadata},
};
use std::collections::HashMap;
use tracing::log::{error, info};

pub async fn read_kv(key_name: &str, cloudflare_config: &ApiClientConfig) -> Option<String> {
    let end_point = ReadKV {
        account_identifier: &cloudflare_config.account_identifier,
        namespace_identifier: &cloudflare_config.namespace_identifier,
        key_name,
    };
    match cloudflare_config.cloudflare_client.send(end_point).await {
        Ok(response) => Some(response),
        Err(error) => {
            info!("Error read_kv: {}", error);
            None
        }
    }
}

pub async fn read_metadata(
    key_name: &str,
    cloudflare_config: &ApiClientConfig,
) -> Option<HashMap<String, String>> {
    let end_point = ReadMetadata {
        account_identifier: &cloudflare_config.account_identifier,
        namespace_identifier: &cloudflare_config.namespace_identifier,
        key_name,
    };
    match cloudflare_config.cloudflare_client.send(end_point).await {
        Ok(response) => {
            if response.success == true {
                Some(response.result.unwrap())
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
        value: &format!(r#""{}""#, value),
        metadata,
    };
    let result = cloudflare_config.cloudflare_client.send(end_point).await;
    info!("result: {:?}", result);
    match result {
        Ok(result) => {
            info!("write: {:?}", result);
            Some(result.success.to_string())
        }
        Err(error) => {
            error!("write error: {}", error);
            None
        }
    }
}

pub async fn delete_kv(key_name: &str, cloudflare_config: ApiClientConfig) -> Option<String> {
    let end_point = DeleteKV {
        account_identifier: &cloudflare_config.account_identifier,
        namespace_identifier: &cloudflare_config.namespace_identifier,
        key_name,
    };
    match cloudflare_config.cloudflare_client.send(end_point).await {
        Ok(result) => {
            info!("delete: {:?}", result);
            Some(result.result.unwrap())
        }
        Err(error) => {
            error!("delete error: {}", error);
            None
        }
    }
}
