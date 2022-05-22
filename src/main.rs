
/*
cargo.toml

[dependencies]
hyper = { version = "0.14", features = ["full"] }
tokio = { version = "1", features = ["full"] }
futures = "0.3"
lazy_static = "1.4.0"
*/

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
