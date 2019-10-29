use async_trait::async_trait;
use hyper::{Response, Request, Body, Error};
use serde_json;

use crate::db::DBService;
use crate::service_registry::ServiceRegistry;
use std::sync::Arc;
use crate::models::{MessageDAO, DAO, Message};

#[async_trait]
pub trait Controller {
    fn create(registry: Arc<ServiceRegistry>) -> Self;
    async fn index(&self, request: Request<Body>) -> Result<Response<Body>, Error>;
}

pub struct MessageController {
    service_registry: Arc<ServiceRegistry>,
}

#[async_trait]
impl Controller for MessageController {
    fn create(registry: Arc<ServiceRegistry>) -> MessageController {
        MessageController {
            service_registry: registry,
        }
    }

    async fn index(&self, request: Request<Body>) -> Result<Response<Body>, Error> {
        let messages = MessageDAO::get_all(&self.service_registry.db).await;
        match messages {
            Ok(items) =>
                Ok(Response::new(Body::from(serde_json::to_string(&items).unwrap()))),
            Err(e) =>
                Ok(Response::builder().status(500).body(Body::empty()).unwrap()),
        }
    }
}
