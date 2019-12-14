use async_trait::async_trait;
use hyper::{Body, Error, Method, Request, Response, StatusCode};
use std::sync::Arc;

use crate::controllers::base::{Controller, ControllerResponse};
use crate::service_registry::ServiceRegistry;


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
//
//    async fn serve(&self, req: Request<Body>) -> Option<ControllerResponse> {
//        match (req.method(), req.uri().path()) {
//            (&Method::GET, "/auth/get_auth") => Some(self.get_auth(req).await),
//            (&Method::GET, "/auth/set_auth") => Some(self.set_auth(req).await),
//            _ => None,
//        }
//    }

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
