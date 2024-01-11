use crate::{connect::EndPoint, endpoints::CloudflareResponse};
use reqwest::Method;

// https://developers.cloudflare.com/api/operations/workers-kv-namespace-delete-key-value-pair
pub struct DeleteKV<'a> {
    account_identifier: &'a str,
    namespace_identifier: &'a str,
    key_name: &'a str,
}

impl<'a> EndPoint<CloudflareResponse<String>> for DeleteKV<'a> {
    fn method(&self) -> Method {
        Method::DELETE
    }

    fn path(&self) -> String {
        format!(
            "/accounts/{}/storage/kv/namespaces/{}/values/{}",
            self.account_identifier, self.namespace_identifier, self.key_name,
        )
    }
}
