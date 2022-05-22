use crate::demo_state::DemoState;

use macroquad::prelude::*;

//Here so I can just copy and paste and start off with a new demo

pub struct DemoEmpty {
    screen_size: Vec2,
}

impl DemoEmpty {
    pub fn new() -> Self {
        let screen_size = vec2(screen_width(), screen_height());
        DemoEmpty {
            screen_size,
        }
    }
}

impl DemoState for DemoBrickBreakState {
    fn process_frame(&mut self) {
        let seconds_delta = get_frame_time();
        let current_screen = vec2(screen_width(), screen_height());
        if current_screen != self.screen_size {
            println!("resize {:?} => {:?}", self.screen_size, current_screen);
            self.screen_size = current_screen;
        }
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_pos = vec2(mouse_x, mouse_y);


        draw_text("Demos", 20.0, last_y, 30.0, DARKGRAY);

    }
}
