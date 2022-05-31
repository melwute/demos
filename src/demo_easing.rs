use std::{convert::TryInto, f32::consts::PI};

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
                duration: 1.5,
                current: 0.0,
            },
        }
    }
}

fn other_lerp(a: f32, b: f32, t: f32) -> f32 {
    (a * t) + (b * (1.0 - t))
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (t * (b - a))
}

//http://sol.gfxile.net/interpolation/
fn smoothstep(t: f32) -> f32 {
    (t) * (t) * (3.0 - 2.0 * (t))
}

fn smootherstep(t: f32) -> f32 {
    (t) * (t) * (t) * ((t) * ((t) * 6.0 - 15.0) + 10.0)
}

fn ease_out_sine(t: f32) -> f32 {
    f32::sin(t * PI / 2.0)
}

fn ease_out_bounce(t: f32) -> f32 {
    const n1: f32 = 7.5625;
    const d1: f32 = 2.75;

    if t < (1.0 / d1) {
        return n1 * t * t;
    } else if t < (2.0 / d1) {
        let t2 = t - (1.5 / d1);
        return (n1 * t2 * t2) + 0.75;
    } else if t < (2.5 / d1) {
        let t2 = t - (2.25 / d1);
        return (n1 * t2 * t2) + 0.9375;
    } else {
        let t2 = t - (2.625 / d1);
        return (n1 * t2 * t2) + 0.984375;
    }
}
impl DemoEasing {
    fn draw_lerp(&self, seconds_delta: f32) {
        let anim = &self.animation;
        let current = {
            let t = anim.current / anim.duration;
            //let t = 1.0 - ((anim.duration - anim.current) / anim.duration);
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
            Color::from_rgba(13, 58, 209, 255),
        );
    }

    fn draw_lerp_other(&self, seconds_delta: f32) {
        let anim = &self.animation;
        /*
        let current = {
            let t = anim.current / anim.duration;
            vec2(
                other_lerp(anim.start.x, anim.end.x, t),
                other_lerp(anim.start.y, anim.end.y, t) - 32.0,
            )
        };
         */

        let current = {
            let t = anim.current / anim.duration;
            let t = ease_out_bounce(t);
            vec2(
                lerp(anim.start.x, anim.end.x, t),
                lerp(anim.start.y, anim.end.y, t) - 32.0,
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

    fn draw_lerp_size(&self, seconds_delta: f32) {
        let anim = &self.animation;
        /*
        let current = {
            let t = anim.current / anim.duration;
            vec2(
                other_lerp(anim.start.x, anim.end.x, t),
                other_lerp(anim.start.y, anim.end.y, t) - 32.0,
            )
        };
         */

        let start = Color::from_rgba(100, 100, 100, 255);
        let end = Color::from_rgba(255, 255, 255, 255);

        let current = {
            let t = anim.current / anim.duration;
            let t = ease_out_bounce(t);
            vec2(lerp(16.0, 32.0, t), lerp(16.0, 32.0, t))
        };

        let pos = {
            let t = anim.current / anim.duration;
            let t = smootherstep(t);
            vec2(
                lerp(anim.start.x, anim.end.x, t),
                lerp(anim.start.y, anim.end.y, t) - 140.0,
            )
        };

        let color = {
            let t = anim.current / anim.duration;
            let t = smootherstep(t);

            Color::from_rgba(
                (lerp(start.r as f32, end.r as f32, t) * 255.0) as u8,
                (lerp(start.g as f32, end.g as f32, t) * 255.0) as i32 as u8,
                (lerp(start.b as f32, end.b as f32, t) * 255.0) as u8,
                255,
            )
        };

        draw_rectangle(pos.x, pos.y, current.x, current.y, color);
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

        let anim = &mut self.animation;
        if anim.current >= anim.duration {
            anim.current = 0.0;
        }

        anim.current += seconds_delta;
        if anim.current >= anim.duration {
            anim.current = anim.duration;
        }

        self.draw_lerp(seconds_delta);
        self.draw_lerp_other(seconds_delta);
        self.draw_lerp_size(seconds_delta);
        /*
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

            println!(
                "{}  {}",
                lerp(anim.start.x, anim.end.y, t),
                other_lerp(anim.start.x, anim.end.y, t),
            );

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
         */
    }
}
