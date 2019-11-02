use async_trait::async_trait;
use hyper::{Response, Request, Body, Error, Method, StatusCode};
use serde_json;

use crate::db::DBService;
use crate::service_registry::ServiceRegistry;
use std::sync::Arc;
use crate::models::{MessageDAO, DAO, Message};

type ControllerResponse = Result<Response<Body>, Error>;

#[async_trait]
pub trait Controller {
    fn create(registry: Arc<ServiceRegistry>) -> Self;
    async fn serve(&self, request: Request<Body>) -> ControllerResponse;
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

    async fn serve(&self, request: Request<Body>) -> ControllerResponse {
        let messages = MessageDAO::get_all(&self.service_registry.db).await;
        match messages {
            Ok(items) =>
                Ok(Response::new(Body::from(serde_json::to_string(&items).unwrap()))),
            Err(e) =>
                Ok(Response::builder().status(500).body(Body::empty()).unwrap()),
        }
    }
}

pub struct AuthController {
    service_registry: Arc<ServiceRegistry>,
}

impl AuthController {
    pub async fn set_auth(&self, request: Request<Body>) -> ControllerResponse {
        self.service_registry.redis.set("auth_key", "auth_success").await
            .and_then(|_| Ok(Response::new(Body::from("ok"))))
            .or_else(|e| Ok(Response::builder().status(500).body(Body::empty()).unwrap()))
    }

    pub async fn get_auth(&self, request: Request<Body>) -> ControllerResponse {
        self.service_registry.redis.get("auth_key").await
            .and_then(|key| Ok(Response::new(Body::from(key.unwrap()))))
            .or_else(|e| Ok(Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::empty()).unwrap()))
    }
}

#[async_trait]
impl Controller for AuthController {
    fn create(registry: Arc<ServiceRegistry>) -> Self {
        AuthController { service_registry: registry }
    }

    async fn serve(&self, req: Request<Body>) -> ControllerResponse {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/auth/get_auth") => {
                self.get_auth(req).await
            },
            (&Method::GET, "/auth/set_auth") => {
                self.set_auth(req).await
            },
            _ => {
                Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body("NOT_FOUND".into())
                    .unwrap())
            }
        }
    }
}