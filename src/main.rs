#![feature(async_closure)]

use tokio;

use hyper;
use hyper::{Body, Response, Server, Request};
use hyper::service::{make_service_fn, service_fn};

use futures::FutureExt;
use tokio_postgres::{NoTls, Error, Row};
use std::sync::Arc;

mod models;
mod db;
mod controllers;
mod service_registry;

use db::{DBBuilder, DB};
use models::{Message, MessageDAO, DAO};
use controllers::{MessageController, Controller};
use service_registry::{ServiceRegistry};

async fn configure_db_service() -> DB {
    let mut db_builder = DBBuilder::new();
    db_builder.set_dbname("chat-api");

    let db = match DB::from_config(&db_builder).await {
        Err(err) => panic!(format!("DBInit error: {:?}", err)),
        Ok(db) => db,
    };

    db
}

async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from("kek")))
}

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let _service_registry = ServiceRegistry { db: configure_db_service().await };
    let service_registry = Arc::new(_service_registry);

    let make_service = make_service_fn(move |_| {
        let service_registry = service_registry.clone();
        async move {
            let service_registry = service_registry.clone();
            Ok::<_, hyper::Error>(service_fn(move |req: Request<Body>| {
                let service_registry = service_registry.clone();
                async move {
                    let service_registry = service_registry.clone();
                    MessageController::create(service_registry).index(req).await
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
