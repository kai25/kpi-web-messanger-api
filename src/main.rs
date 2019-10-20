use std::net::SocketAddr;
use hyper::{
    Server,
    Request,
    Body,
    Response,
    rt::run,
};
use hyper::service::service_fn;

use futures::{
    compat::Future01CompatExt,
    future::{FutureExt, TryFutureExt, lazy, poll_fn},
};


async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from("hello, world!")))
}

async fn run_server(addr: SocketAddr) {
    println!("Running on {}", addr.to_string());

    let serve_future = Server::bind(&addr)
        .serve(|| service_fn(|req| serve_req(req).boxed().compat()));

    if let Err(e) = serve_future.compat().await {
        eprintln!("Server error: {}", e);
    }
}

fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let run_server_future = run_server(addr);
    run(run_server_future.unit_error().boxed().compat());
}
