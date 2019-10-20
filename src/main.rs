use tokio;

use hyper;
use hyper::{Body, Response, Server, Request};
use hyper::service::{make_service_fn, service_fn};

use futures::FutureExt;
use tokio_postgres::{NoTls, Error, Row};

async fn query() -> Result<String, Error> {
    // Connect to the database.
    let (mut client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=postgres", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    let connection = connection.map(|r| {
        if let Err(e) = r {
            eprintln!("connection error: {}", e);
        }
    });

    tokio::spawn(connection);


    let rows: Vec<Row> = client
        .query("select * from test_table limit 1" ,&[])
        .await?;

    // Now we can check that we got back the same string we sent over.
    let value: &str = rows[0].get(0);
    Ok(value.to_owned())
}

async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("query {:?}", query().await);
    Ok(Response::new(Body::from("hello, world!")))
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
