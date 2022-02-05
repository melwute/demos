use crate::demo_state::DemoState;
use instant::SystemTime;
use macroquad::prelude::*;
use std::f64;

//inspired by
//#https://github.com/s9w/oof/blob/master/demos/snow_demo.cpp

struct SnowFlake {
    x: f32,
    y: f32,
    z_distance: f32,
}

impl SnowFlake {}

pub struct SnowingState {
    max_speed: f32,
    grid_size_px: u32,
    grid_width: u32,
    grid_height: u32,
    min_brightness: f32,
    max_brightness: f32,
    spawn_chance: f32,
    snowflakes: Vec<SnowFlake>,
    height_levels: Vec<f32>,
}

impl SnowingState {
    pub fn new(screen_width_px: u32, screen_height_px: u32) -> Self {
        let grid_size_px: u32 = 20;
        let grid_width = screen_width_px / grid_size_px;
        let grid_height: u32 = screen_height_px / grid_size_px;

        let height_levels = vec![screen_height_px as f32; grid_width as usize];

        let mut state = SnowingState {
            max_speed: 120.0,
            grid_size_px: grid_size_px,
            grid_width: grid_width,
            grid_height: grid_height,
            min_brightness: 0.3,
            max_brightness: 1.0,
            spawn_chance: 0.8,
            snowflakes: Vec::new(),
            height_levels,
        };

        for i in (0..1) {
            state.new_flake();
        }

        return state;
    }

    pub fn new_flake(&mut self) {
        let x: f32 = rand::gen_range(0, self.grid_width) as f32 * self.grid_size_px as f32;
        let y: f32 = -(self.grid_size_px as f32); //put it just off screen
        let z: f32 = rand::gen_range(0.25, 1.0);

        let flake = SnowFlake {
            x,
            y,
            z_distance: z,
        };

        self.snowflakes.push(flake);
    }
}
impl DemoState for SnowingState {
    fn process_frame(&mut self) {
        let (w, h) = (screen_width(), screen_height());
        draw_text(&format!("{},{}", w, h), 20.0, 20.0, 30.0, DARKGRAY);
        let roll = rand::gen_range(0.0, 1.0);

        if roll > self.spawn_chance {
            self.new_flake();
        }

        let delta = get_frame_time();

        for flake in self.snowflakes.iter_mut() {
            let column = (flake.x / self.grid_size_px as f32).floor() as usize;

            let snow_height = self.height_levels[column];

            let desired_y = flake.y + (delta * self.max_speed * flake.z_distance);

            if snow_height >= (flake.y + self.grid_size_px as f32) {
                flake.y = flake.y + (delta * self.max_speed * flake.z_distance);
            } else if flake.z_distance > 0.5 {
                let new_snow_height = snow_height.min(flake.y);
                self.height_levels[column] = new_snow_height;
            }
        }

        let delta_brightness = self.max_brightness - self.min_brightness;
        for flake in self.snowflakes.iter() {
            let brightness = flake.z_distance * delta_brightness + self.min_brightness;
            let color = Color::new(brightness, brightness, brightness, 1.0);
            draw_rectangle(
                flake.x,
                flake.y,
                self.grid_size_px as f32,
                self.grid_size_px as f32,
                color,
            );
        }
    }
}
