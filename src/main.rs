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
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
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

/*
cargo.toml

[dependencies]
hyper = { version = "0.14", features = ["full"] }
tokio = { version = "1", features = ["full"] }
futures = "0.3"
lazy_static = "1.4.0"
*/

/*
use crate::demo_brickbreak::DemoBrickBreakState;
use crate::demo_foragerclone::DemoForagerClone;
use crate::demo_roguelike::DemoRoguelike;
use crate::demo_snowflake::SnowingState;
use crate::demo_tabtargetrpg::DemoTabTargetRpg;

use demo_state::DemoState;
use instant::SystemTime;
use macroquad::prelude::*;

mod demo_brickbreak;
mod demo_foragerclone;
mod demo_roguelike;
mod demo_snowflake;
mod demo_state;
mod demo_tabtargetrpg;

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

#[macroquad::main("BasicShapes")]
async fn main() {
    rand::srand(get_epoch_ms() as u64);
    let mut state = ApplicationState {
        current_demo: Some(Box::new(DemoForagerClone::new().await)),
    };
    loop {
        state.process().await;
        next_frame().await
    }
}

pub struct ApplicationState {
    current_demo: Option<Box<dyn DemoState>>,
}
impl ApplicationState {
    async fn process(&mut self) {
        if is_key_released(KeyCode::Escape) {
            self.current_demo = None
        }

        if let Some(demo) = &mut self.current_demo {
            demo.process_frame();
            return;
        }

        self.mainscreen_process_frame().await;
    }

    //maybe this should be its own DemoState...
    async fn mainscreen_process_frame(&mut self) {
        let horizontal_spacing = 40.0;
        let mut last_y = 30.0;

        draw_text("Demos", 20.0, last_y, 30.0, DARKGRAY);
        last_y += horizontal_spacing;

        draw_text("Press Esc to end any demo.", 20.0, last_y, 30.0, DARKGRAY);
        last_y += horizontal_spacing;

        draw_text("1) Snowflakes ", 20.0, last_y, 30.0, DARKGRAY);
        last_y += horizontal_spacing;

        draw_text("2) BrickBreak ", 20.0, last_y, 30.0, DARKGRAY);
        last_y += horizontal_spacing;

        draw_text("3) Tab Target ", 20.0, last_y, 30.0, DARKGRAY);
        last_y += horizontal_spacing;

        draw_text("4) Roguelike grid ", 20.0, last_y, 30.0, DARKGRAY);
        last_y += horizontal_spacing;

        draw_text("5) Forager clone ", 20.0, last_y, 30.0, DARKGRAY);
        last_y += horizontal_spacing;

        if is_key_released(KeyCode::Key1) {
            self.current_demo = Some(Box::new(SnowingState::new(
                screen_width() as u32,
                screen_height() as u32,
            )));
        }

        if is_key_released(KeyCode::Key2) {
            self.current_demo = Some(Box::new(DemoBrickBreakState::new()));
        }

        if is_key_released(KeyCode::Key3) {
            self.current_demo = Some(Box::new(DemoTabTargetRpg::new().await));
        }

        if is_key_released(KeyCode::Key4) {
            self.current_demo = Some(Box::new(DemoRoguelike::new().await));
        }

        if is_key_released(KeyCode::Key5) {
            self.current_demo = Some(Box::new(DemoForagerClone::new().await));
        }
    }
}

*/
