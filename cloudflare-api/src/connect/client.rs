use crate::connect::{AuthClient, Credentials};
use reqwest::{header, Client, Error, Method, RequestBuilder};
use serde::Deserialize;

use super::EndPoint;

pub struct HttpApiClient {
    client: Client,
}

impl HttpApiClient {
    pub fn new(client: Client) -> HttpApiClient {
        HttpApiClient { client }
    }

    pub async fn send<T>(&self, end_point: impl EndPoint<T>) -> Result<T, Error>
    where
        for<'de> T: Deserialize<'de>,
    {
        let mut request_builder = self
            .request_builder(end_point.url().as_str(), end_point.method())
            .header(header::CONTENT_TYPE, end_point.content_type().as_ref());
        if end_point.multipart().is_some() {
            request_builder = request_builder.multipart(end_point.multipart().unwrap());
        }
        if end_point.body().is_some() {
            request_builder = request_builder.body(end_point.body().unwrap());
        }

        let body = request_builder.send().await?.json::<T>().await?;
        Ok(body)
    }

    fn request_builder(&self, url: &str, method: Method) -> RequestBuilder {
        match method {
            Method::GET => self.client.get(url),
            Method::POST => self.client.post(url),
            Method::PUT => self.client.put(url),
            Method::DELETE => self.client.delete(url),
            _ => panic!("Not supported"),
        }
    }
}

impl AuthClient for HttpApiClient {
    fn client(&mut self, credentials: &Credentials) -> &Self {
        let mut headers = header::HeaderMap::new();
        for header in credentials.headers() {
            let mut auth_header = header::HeaderValue::from_str(&header.1).unwrap();
            auth_header.set_sensitive(true);
            headers.insert(header.0, auth_header);
        }
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/json"),
        );
        // TODO: Replace expect() and handle error gracefully
        self.client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Could not initialize connection with Cloudflare V4 API!");
        self
    }
}
