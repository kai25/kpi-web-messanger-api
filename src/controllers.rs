use serde_json;

use async_trait::async_trait;
use hyper::{Body, Error, Method, Request, Response, StatusCode};
use std::sync::Arc;

use crate::models_::{Message, MessageDAO, DAO};
use crate::service_registry::ServiceRegistry;

type ControllerResponse = Result<Response<Body>, Error>;

#[async_trait]
pub trait Controller {
    fn new(registry: Arc<ServiceRegistry>) -> Self;
    async fn serve(&self, request: Request<Body>) -> ControllerResponse;
    async fn create(&self, request: Request<Body>) -> ControllerResponse;
    async fn delete(&self, request: Request<Body>) -> ControllerResponse;
    async fn update(&self, request: Request<Body>) -> ControllerResponse;
    async fn get_all(&self, request: Request<Body>) -> ControllerResponse;
    async fn get_by_id(&self, request: Request<Body>) -> ControllerResponse;
}

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

    async fn serve(&self, request: Request<Body>) -> ControllerResponse {
        match (request.method(), request.uri().path()) {
            (&Method::GET, "/message") => self.get_all(request).await,
            (&Method::POST, "/message") => self.create(request).await,
            _ => Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("NOT_FOUND".into())
                .unwrap()),
        }
    }

    async fn create(&self, request: Request<Body>) -> Result<Response<Body>, Error> {
        let json_body = hyper::body::to_bytes(request.into_body()).await?;
        println!("{:?}", &json_body);
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

    async fn delete(&self, request: Request<Body>) -> Result<Response<Body>, Error> {
        unimplemented!()
    }

    async fn update(&self, request: Request<Body>) -> Result<Response<Body>, Error> {
        unimplemented!()
    }

    async fn get_all(&self, request: Request<Body>) -> Result<Response<Body>, Error> {
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

pub struct AuthController {
    service_registry: Arc<ServiceRegistry>,
}

impl AuthController {
    pub async fn set_auth(&self, request: Request<Body>) -> ControllerResponse {
        self.service_registry
            .redis
            .set("auth_key", "auth_success")
            .await
            .and_then(|_| Ok(Response::new(Body::from("ok"))))
            .or_else(|e| Ok(Response::builder().status(500).body(Body::empty()).unwrap()))
    }

    pub async fn get_auth(&self, request: Request<Body>) -> ControllerResponse {
        self.service_registry
            .redis
            .get("auth_key")
            .await
            .and_then(|key| Ok(Response::new(Body::from(key.unwrap()))))
            .or_else(|e| {
                Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::empty())
                    .unwrap())
            })
    }
}

#[async_trait]
impl Controller for AuthController {
    fn new(registry: Arc<ServiceRegistry>) -> Self {
        AuthController {
            service_registry: registry,
        }
    }

    async fn serve(&self, req: Request<Body>) -> ControllerResponse {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/auth/get_auth") => self.get_auth(req).await,
            (&Method::GET, "/auth/set_auth") => self.set_auth(req).await,
            _ => Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("NOT_FOUND".into())
                .unwrap()),
        }
    }

    async fn create(&self, request: Request<Body>) -> Result<Response<Body>, Error> {
        unimplemented!()
    }

    async fn delete(&self, request: Request<Body>) -> Result<Response<Body>, Error> {
        unimplemented!()
    }

    async fn update(&self, request: Request<Body>) -> Result<Response<Body>, Error> {
        unimplemented!()
    }

    async fn get_all(&self, request: Request<Body>) -> Result<Response<Body>, Error> {
        unimplemented!()
    }

    async fn get_by_id(&self, request: Request<Body>) -> Result<Response<Body>, Error> {
        unimplemented!()
    }
}
