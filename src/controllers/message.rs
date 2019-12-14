use serde_json;

use async_trait::async_trait;
use hyper::{Body, Error, Method, Request, Response, StatusCode};
use std::sync::Arc;

use crate::controllers::base::{Controller, ControllerResponse};

use crate::models::base::{DAO};
use crate::models::message::{MessageDAO, Message};

use crate::service_registry::ServiceRegistry;

pub struct MessageController {
    service_registry: Arc<ServiceRegistry>,
}

#[async_trait]
impl Controller for MessageController {
    fn new(registry: Arc<ServiceRegistry>) -> MessageController {
        MessageController {
            service_registry: registry,
        }
    }

    async fn create(&self, request: Request<Body>) -> ControllerResponse {
        let json_body = hyper::body::to_bytes(request.into_body()).await?;
        let message: Message = serde_json::from_slice(&json_body).unwrap();
        let id = MessageDAO::create(&self.service_registry.db, &message).await;

        id.and_then(|id| Ok(Response::builder().body(format!("{}", id).into()).unwrap()))
            .or_else(|err| {
                eprintln!("{:?}", err);
                Ok(Response::builder()
                    .status(500)
                    .body("Error".into())
                    .unwrap())
            })
    }

    async fn delete(&self, request: Request<Body>) -> ControllerResponse {
        unimplemented!()
    }

    async fn update(&self, request: Request<Body>) -> ControllerResponse {
        unimplemented!()
    }

    async fn get_all(&self, request: Request<Body>) -> ControllerResponse {
        let messages = MessageDAO::get_all(&self.service_registry.db).await;
        messages
            .and_then(|items| Ok(Response::new(serde_json::to_string(&items).unwrap().into())))
            .or_else(|err| {
                Ok(Response::builder()
                    .status(500)
                    .body("Error".into())
                    .unwrap())
            })
    }

    async fn get_by_id(&self, request: Request<Body>) -> Result<Response<Body>, Error> {
        unimplemented!()
    }
}
