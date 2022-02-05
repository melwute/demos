use crate::demo_snowflake::SnowingState;
use demo_state::DemoState;
use instant::SystemTime;
use macroquad::prelude::*;

mod demo_snowflake;
mod demo_state;

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

#[macroquad::main("BasicShapes")]
async fn main() {
    rand::srand(get_epoch_ms() as u64);
    let mut state = SnowingState::new(screen_width() as u32, screen_height() as u32);

    let mut demo: Box<dyn DemoState> = Box::new(state);

    loop {
        demo.process_frame();
        next_frame().await
    }
}
