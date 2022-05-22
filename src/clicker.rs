
use futures::TryStreamExt as _;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use hyper::{Method, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
#[macro_use]
extern crate lazy_static;

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

async fn hello_basic(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/click") => {
            let clicks = get_clicks();
            *response.body_mut() = Body::from(format!("clicks {}", clicks));
        }
        (&Method::POST, "/click") => {
            inc_clicks(1);
            let clicks = get_clicks();
            *response.body_mut() = Body::from(format!("clicks {}", clicks));
        }
        (&Method::POST, "/echo/reverse") => {
            //lets just consume the entire thing before hand.
            let full_body = hyper::body::to_bytes(req.into_body()).await?;
            let reversed = full_body.iter().rev().cloned().collect::<Vec<u8>>();

            *response.body_mut() = reversed.into();
        }
        (&Method::POST, "/echo/uppercase") => {
            //efficent streaming
            let mapping = req.into_body().map_ok(|chunk| {
                chunk
                    .iter()
                    .map(|byte| byte.to_ascii_uppercase())
                    .collect::<Vec<u8>>()
            });

            *response.body_mut() = Body::wrap_stream(mapping);
        }
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}

use std::sync::Mutex;

lazy_static! {
    static ref global_clicks: Mutex<u128> = Mutex::new(0 as u128);
}

fn get_clicks() -> u128 {
    let clicks = global_clicks.lock().unwrap();
    return *clicks;
}

fn inc_clicks(amount: i32) {
    let mut clicks = global_clicks.lock().unwrap();
    *clicks += amount as u128;
}

#[tokio::main]
async fn main() {
    println!("Number of clicks {}", get_clicks());

    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_basic))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

/*
cargo.toml

[dependencies]
hyper = { version = "0.14", features = ["full"] }
tokio = { version = "1", features = ["full"] }
futures = "0.3"
lazy_static = "1.4.0"
*/