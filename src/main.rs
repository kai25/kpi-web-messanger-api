#![feature(async_closure)]

use tokio;
use serde;

use hyper;
use hyper::{Body, Response, Server, Request};
use hyper::service::{make_service_fn, service_fn};

use futures::FutureExt;
use tokio_postgres::{NoTls, Error, Row};
use std::sync::Arc;

mod models;
mod db;
mod redis_service;
mod controllers;
mod service_registry;

use db::{DBBuilder, DBService};
use redis_service::RedisService;
use models::{Message, MessageDAO, DAO};
use controllers::{MessageController, AuthController, Controller};
use service_registry::{ServiceRegistry};

async fn configure_db_service() -> DBService {
    let mut db_builder = DBBuilder::new();
    db_builder.set_dbname("chat-api");

    let db = match DBService::from_config(&db_builder).await {
        Err(err) => panic!(format!("DBInit error: {:?}", err)),
        Ok(db) => db,
    };

    db
}

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let _service_registry = ServiceRegistry {
        db: configure_db_service().await,
        redis: RedisService::configure("redis://localhost:6379").await,
    };
    let service_registry = Arc::new(_service_registry);

    let make_service = make_service_fn(move |_| {
        let service_registry = service_registry.clone();
        async move {
            let service_registry = service_registry.clone();
            Ok::<_, hyper::Error>(service_fn(move |req: Request<Body>| {
                let service_registry = service_registry.clone();
                async move {
                    let service_registry = service_registry.clone();
                    AuthController::create(service_registry).serve(req).await
                }
            }))
        }
    });

    let server = Server::bind(&addr)
        .serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
