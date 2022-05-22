use crate::demo_state::DemoState;

use macroquad::prelude::*;

//Here so I can just copy and paste and start off with a new demo
//https://www.youtube.com/watch?v=Fy0aCDmgnxg

pub struct DemoEasing {
    screen_size: Vec2,
}

impl DemoEasing {
    pub fn new() -> Self {
        let screen_size = vec2(screen_width(), screen_height());
        DemoEasing {
            screen_size,
        }
    }
}

impl DemoState for DemoEasing {
    fn process_frame(&mut self) {
        let seconds_delta = get_frame_time();
        let current_screen = vec2(screen_width(), screen_height());
        if current_screen != self.screen_size {
            println!("resize {:?} => {:?}", self.screen_size, current_screen);
            self.screen_size = current_screen;
        }
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_pos = vec2(mouse_x, mouse_y);


        draw_text("Easing", 20.0, 90.0, 30.0, DARKGRAY);

    }
}
