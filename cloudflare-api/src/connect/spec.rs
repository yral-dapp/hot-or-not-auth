use reqwest::multipart::Form;
use std::{borrow::Cow, collections::HashMap};
use url::Url;

pub trait EndPoint<T> {
    fn method(&self) -> reqwest::Method;

    fn path(&self) -> String;

    #[inline]
    fn query(&self) -> Option<HashMap<String, String>> {
        None
    }

    #[inline]
    fn serialize_query(&self) -> Option<String> {
        if self.query().is_none() {
            None
        } else {
            serde_urlencoded::to_string(self.query()).ok()
        }
    }

    #[inline]
    fn body(&self) -> Option<String> {
        None
    }

    #[inline]
    fn multipart(&self) -> Option<Form> {
        None
    }

    #[inline]
    fn content_type(&self) -> Cow<'static, str> {
        Cow::Borrowed("application/json")
    }

    // TODO: make URL configurable
    fn url(&self) -> String {
        let mut url = Url::parse("https://api.cloudflare.com")
            .unwrap()
            .join(&format!("/client/v4{}", &self.path()))
            .unwrap();
        url.set_query(self.serialize_query().as_deref());
        url.to_string()
    }
}
