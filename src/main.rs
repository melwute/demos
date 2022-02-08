use crate::demo_brickbreak::DemoBrickBreakState;
use crate::demo_snowflake::SnowingState;
use demo_state::DemoState;
use instant::SystemTime;
use macroquad::prelude::*;

mod demo_brickbreak;
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
    let mut state = ApplicationState { current_demo: None };
    loop {
        state.process();
        next_frame().await
    }
}

pub struct ApplicationState {
    current_demo: Option<Box<dyn DemoState>>,
}
impl ApplicationState {
    fn process(&mut self) {
        if is_key_released(KeyCode::Escape) {
            self.current_demo = None;
        }

        if let Some(demo) = &mut self.current_demo {
            demo.process_frame();
            return;
        }

        self.mainscreen_process_frame();
    }

    //maybe this should be its own DemoState...
    fn mainscreen_process_frame(&mut self) {
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

        if is_key_released(KeyCode::Key1) {
            self.current_demo = Some(Box::new(SnowingState::new(
                screen_width() as u32,
                screen_height() as u32,
            )));
        }

        if is_key_released(KeyCode::Key2) {
            self.current_demo = Some(Box::new(DemoBrickBreakState::new()));
        }
    }
}
