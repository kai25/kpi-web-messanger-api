use serde_json;

use async_trait::async_trait;
use hyper::{Body, Error, Method, Request, Response, StatusCode};
use std::sync::Arc;

use crate::models::base::{DAO};
use crate::models::message::{MessageDAO, Message};
use crate::models::user::{UserDAO, User};

use crate::service_registry::ServiceRegistry;

pub type ControllerResponse = Result<Response<Body>, Error>;

#[async_trait]
pub trait Controller {
    fn new(registry: Arc<ServiceRegistry>) -> Self;
    async fn create(&self, request: Request<Body>) -> ControllerResponse;
    async fn delete(&self, request: Request<Body>) -> ControllerResponse;
    async fn update(&self, request: Request<Body>) -> ControllerResponse;
    async fn get_all(&self, request: Request<Body>) -> ControllerResponse;
    async fn get_by_id(&self, request: Request<Body>) -> ControllerResponse;
}
