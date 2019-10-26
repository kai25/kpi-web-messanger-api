use tokio;

use hyper;
use hyper::{Body, Response, Server, Request};
use hyper::service::{make_service_fn, service_fn};

use futures::FutureExt;
use tokio_postgres::{NoTls, Error, Row};

mod models;
mod db;

use db::{DBBuilder, DB};
use models::{Message, MessageDAO, DAO};

async fn query() -> Result<String, Error> {
    // Connect to the database.
    let mut db_builder = DBBuilder::new();
    db_builder.set_dbname("chat-api");

    let db = match DB::from_config(&db_builder).await {
        Err(err) => panic!(format!("DBInit error: {:?}", err)),
        Ok(db) => db,
    };

//    MessageDAO::create(&db, &Message { id: 0, text: "another one".to_owned() }).await?;

    let total_text = MessageDAO::get_all(&db)
        .await?
        .into_iter()
        .fold(String::from("List of messages"), |text, message| format!("{}\n{}", text, message));

    Ok(total_text)
}

async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let text = query().await.unwrap();
    Ok(Response::new(Body::from(text)))
}

#[tokio::main]
async fn main() {
    // Construct our SocketAddr to listen on...
    let addr = ([127, 0, 0, 1], 3000).into();

    // And a MakeService to handle each connection...
    let make_service = make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(serve_req))
    });

    // Then bind and serve...
    let server = Server::bind(&addr)
        .serve(make_service);

    // Finally, spawn `server` onto an Executor...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
