use async_trait::async_trait;
use hyper::{Response, Request, Body, Error};
use crate::db::DB;
use crate::service_registry::ServiceRegistry;
use std::sync::Arc;

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
        Ok(Response::new(Body::from("hello from message controller!")))
    }
}
