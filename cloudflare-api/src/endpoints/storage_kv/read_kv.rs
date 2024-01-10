use crate::connect::EndPoint;
use reqwest::Method;

// https://developers.cloudflare.com/api/operations/workers-kv-namespace-read-key-value-pair
pub struct ReadKV<'a> {
    account_identifier: &'a str,
    namespace_identifier: &'a str,
    key_name: &'a str,
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
