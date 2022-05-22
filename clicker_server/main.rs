use futures::TryStreamExt as _;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use hyper::{Method, StatusCode};
use std::convert::Infallible;
use std::error;
use std::net::SocketAddr;
#[macro_use]
extern crate lazy_static;

async fn hello_basic(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let static_file = hyper_staticfile::Static::new("");

    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/click") => {
            let clicks = get_clicks();

            *response.body_mut() = Body::from(format!("{}\"clicks\":{}{}", '{', clicks, '}'));
        }
        (&Method::POST, "/click") => {
            inc_clicks(1);
            let clicks = get_clicks();

            *response.body_mut() = Body::from(format!("{}\"clicks\":{}{}", '{', clicks, '}'));
        }
        _ => {
            let file_response = static_file.serve(req).await;

            //I for the life of me cannot map this error correctly
            match file_response {
                Ok(response) => {
                    return Ok(response);
                }
                Err(err) => {
                    println!("Error serving file {:?}", err);
                    *response.status_mut() = StatusCode::NOT_FOUND;
                }
            }
            //*response.status_mut() = StatusCode::NOT_FOUND;
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
