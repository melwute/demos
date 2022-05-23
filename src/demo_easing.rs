use crate::demo_state::DemoState;

use macroquad::prelude::*;

//Here so I can just copy and paste and start off with a new demo
//https://www.youtube.com/watch?v=Fy0aCDmgnxg

pub struct Brick {
    color: Color,
    position: Vec2,
    dimension: Vec2,
}

#[derive(Debug)]
struct Animation {
    start: Vec2,
    end: Vec2,
    duration: f32, //seconds TODO duration instead of f32?
    current: f32,
}

pub struct DemoEasing {
    screen_size: Vec2,
    brick: Brick,
    animation: Animation,
}

impl DemoEasing {
    pub fn new() -> Self {
        let screen_size = vec2(screen_width(), screen_height());

        let start = vec2(0.0 + 32.0, screen_size.y / 2.0);
        let end = vec2(screen_size.x / 2.0, screen_size.y / 2.0);

        DemoEasing {
            screen_size,
            brick: Brick {
                color: Color::from_rgba(208, 58, 209, 255),
                position: start,
                dimension: vec2(32.0, 32.0),
            },
            animation: Animation {
                start,
                end,
                duration: 1.0,
                current: 0.0,
            },
        }
    }
}
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (t * (b - a))
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

        if is_key_released(KeyCode::KpAdd) {
            self.animation.duration += 0.25;
        }

        if is_key_released(KeyCode::KpSubtract) {
            self.animation.duration -= 0.25;
        }

        draw_text(
            &format!("duration: {:?}", self.animation.duration),
            20.0,
            70.0,
            30.0,
            DARKGRAY,
        );
        draw_text(
            &format!("current: {:?}", self.animation.current),
            20.0,
            90.0,
            30.0,
            DARKGRAY,
        );

        let current = {
            let anim = &mut self.animation;
            if anim.current >= anim.duration {
                anim.current = 0.0;
            }

            anim.current += seconds_delta;
            if anim.current >= anim.duration {
                anim.current = anim.duration;
            }
            let t = 1.0 - ((anim.duration - anim.current) / anim.duration);

            vec2(
                lerp(anim.start.x, anim.end.x, t),
                lerp(anim.start.y, anim.end.y, t),
            )
        };

        draw_rectangle(
            current.x,
            current.y,
            self.brick.dimension.x,
            self.brick.dimension.y,
            self.brick.color,
        );
    }
}
