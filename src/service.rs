use r2d2::Pool;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::{convert::Infallible, net::SocketAddr};


async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World")))
}


pub async fn run(addr: SocketAddr, sql_pool: Pool<PostgresConnectionManager<NoTls>>) {
    // And a MakeService to handle each connection...
    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
