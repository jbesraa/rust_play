use crate::server;
use crate::Server;
use crate::State;
use futures::{executor::block_on, prelude::*};
use http_service::{HttpService, Response};
use http_types::Request;
use serde::de::DeserializeOwned;
use std::pin::Pin;

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
