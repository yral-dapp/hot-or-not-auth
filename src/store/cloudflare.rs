use cloudflare_api::{
    connect::ApiClientConfig,
    endpoints::storage_kv::{DeleteKV, ReadKV, ReadMetadata, WriteKVWithMetadata},
};
use std::collections::HashMap;
use tracing::log::{error, info, warn};

pub async fn read_kv(key_name: &str, cloudflare_config: &ApiClientConfig) -> Option<String> {
    let end_point = ReadKV {
        account_identifier: &cloudflare_config.account_identifier,
        namespace_identifier: &cloudflare_config.namespace_identifier,
        key_name,
    };
    match cloudflare_config.cloudflare_client.send(end_point).await {
        Ok(response) => Some(response),
        Err(error) => {
            warn!("Error read_kv: {}", error);
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
            error!("Error read_metadata: {}", error);
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
