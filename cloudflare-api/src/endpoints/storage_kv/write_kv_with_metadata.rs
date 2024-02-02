use crate::{connect::EndPoint, endpoints::CloudflareResponse};
use reqwest::multipart::Form;
use std::{borrow::Cow, collections::HashMap};

// https://developers.cloudflare.com/api/operations/workers-kv-namespace-write-key-value-pair-with-metadata
#[derive(Debug)]
pub struct WriteKVWithMetadata<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub key_name: &'a str,
    pub value: &'a str,
    pub metadata: HashMap<&'a str, &'a str>,
}

impl<'a> EndPoint<CloudflareResponse<String>> for WriteKVWithMetadata<'a> {
    fn method(&self) -> reqwest::Method {
        reqwest::Method::PUT
    }

    fn path(&self) -> String {
        format!(
            "/accounts/{}/storage/kv/namespaces/{}/values/{}",
            self.account_identifier, self.namespace_identifier, self.key_name
        )
    }

    #[inline]
    fn body(&self) -> Option<String> {
        None
    }

    #[inline]
    fn multipart(&self) -> Option<Form> {
        let form = Form::new()
            .text("metadata", serde_json::to_string(&self.metadata).unwrap())
            .text("value", self.value.to_owned());
        Some(form)
    }

    #[inline]
    fn content_type(&self) -> Cow<'static, str> {
        Cow::Borrowed("multipart/form-data")
    }
}
