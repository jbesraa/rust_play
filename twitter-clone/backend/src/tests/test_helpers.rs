use crate::server;
use crate::Server;
use crate::State;
use futures::{executor::block_on, prelude::*};
use http_service::{HttpService, Response};
use http_types::{Method, Url};
use serde::de::DeserializeOwned;
use std::pin::Pin;

pub use assert_json_diff::assert_json_include;
pub use http_types::Request;
pub use serde_json::{json, Value};

pub async fn test_server() -> TestBackend<Server<State>> {
    let server = server().await;
    make_server(server).unwrap()
}

#[derive(Debug)]
pub struct TestBackend<T: HttpService> {
    service: T,
    connection: T::Connection,
}

impl<T: HttpService> TestBackend<T> {
    fn wrap(service: T) -> Result<Self, <T::ConnectionFuture as TryFuture>::Error> {
        let connection = block_on(service.connect().into_future())?;
        Ok(Self {
            service,
            connection,
        })
    }
    pub fn simulate(
        &mut self,
        req: Request,
    ) -> Result<Response, <T::ResponseFuture as TryFuture>::Error> {
        block_on(
            self.service
                .respond(self.connection.clone(), req)
                .into_future(),
        )
    }
}

pub fn make_server<T: HttpService>(
    service: T,
) -> Result<TestBackend<T>, <T::ConnectionFuture as TryFuture>::Error> {
    TestBackend::wrap(service)
}

pub trait BodyJson {
    fn body_json<T: DeserializeOwned>(
        self,
    ) -> Pin<Box<dyn Future<Output = Result<T, Box<dyn std::error::Error>>>>>;
}

impl BodyJson for Response {
    fn body_json<T: DeserializeOwned>(
        self,
    ) -> Pin<Box<dyn Future<Output = Result<T, Box<dyn std::error::Error>>>>> {
        Box::pin(async move {
            let body = self.body_string().await?;
            println!("body = {}", body);
            Ok(serde_json::from_str(&body)?)
        })
    }
}

pub trait MakeRequestBuilder {
    fn build() -> RequestBuilder;
}

impl MakeRequestBuilder for Request {
    fn build() -> RequestBuilder {
        RequestBuilder {
            method: None,
            url: None,
        }
    }
}

#[derive(Debug, Default)]
pub struct RequestBuilder {
    method: Option<Method>,
    url: Option<String>,
}

impl RequestBuilder {
    pub fn get(mut self) -> Self {
        self.method = Some(Method::Get);
        self
    }
    pub fn post(mut self) -> Self {
        self.method = Some(Method::Post);
        self
    }
    pub fn Path(mut self) -> Self {
        self.method = Some(Method::Patch);
        self
    }
    pub fn put(mut self) -> Self {
        self.method = Some(Method::Put);
        self
    }
    pub fn delete(mut self) -> Self {
        self.method = Some(Method::Delete);
        self
    }
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }
    pub fn send(self, server: &mut TestBackend<Server<State>>) -> Response {
        let url = Url::parse(&format!("http://example.com{}", self.url.unwrap())).unwrap();
        let req = Request::new(Method::Get, url);
        server.simulate(req).unwrap()
    }
}
