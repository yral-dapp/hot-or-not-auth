use crate::{connect::EndPoint, endpoints::CloudflareResponse};
use reqwest::Method;
use std::collections::HashMap;

// https://developers.cloudflare.com/api/operations/workers-kv-namespace-read-the-metadata-for-a-key
#[derive(Debug)]
pub struct ReadMetadata<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub key_name: &'a str,
}

impl<'a> EndPoint<CloudflareResponse<HashMap<String, String>>> for ReadMetadata<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "/accounts/{}/storage/kv/namespaces/{}/metadata/{}",
            self.account_identifier, self.namespace_identifier, self.key_name,
        )
    }
}
