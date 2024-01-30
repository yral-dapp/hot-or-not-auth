use crate::connect::EndPoint;
use reqwest::Method;

// https://developers.cloudflare.com/api/operations/workers-kv-namespace-read-key-value-pair
#[derive(Debug)]
pub struct ReadKV<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub key_name: &'a str,
}

impl<'a> EndPoint<String> for ReadKV<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!(
            "/accounts/{}/storage/kv/namespaces/{}/values/{}",
            self.account_identifier, self.namespace_identifier, self.key_name,
        )
    }
}
