use instant::SystemTime;
use macroquad::prelude::*;
use std::f64;

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

//inspired by
//#https://github.com/s9w/oof/blob/master/demos/snow_demo.cpp

struct SnowFlake {
    x: f32,
    y: f32,
    z_distance: f32,
}

impl SnowFlake {}

struct SnowingState {
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

#[macroquad::main("BasicShapes")]
async fn main() {
    rand::srand(get_epoch_ms() as u64);
    let x: f32 = rand::gen_range(0., 400.);
    let y: f32 = rand::gen_range(0., 800.);

    let mut state = SnowingState::new(screen_width() as u32, screen_height() as u32);

    let (w, h) = (screen_width(), screen_height());

    loop {
        draw_text(&format!("{},{}", w, h), 20.0, 20.0, 30.0, DARKGRAY);
        let roll = rand::gen_range(0.0, 1.0);

        if roll > state.spawn_chance {
            state.new_flake();
        }

        let delta = get_frame_time();

        for flake in state.snowflakes.iter_mut() {
            let column = (flake.x / state.grid_size_px as f32).floor() as usize;

            let snow_height = state.height_levels[column];

            let desired_y = flake.y + (delta * state.max_speed * flake.z_distance);

            if snow_height >= (flake.y + state.grid_size_px as f32) {
                flake.y = flake.y + (delta * state.max_speed * flake.z_distance);
            } else if flake.z_distance > 0.5 {
                let new_snow_height = snow_height.min(flake.y);
                state.height_levels[column] = new_snow_height;
            }
        }

        let delta_brightness = state.max_brightness - state.min_brightness; 
        for flake in state.snowflakes.iter() {
            let brightness = flake.z_distance * delta_brightness + state.min_brightness;
            let color = Color::new(brightness, brightness, brightness, 1.0);
            draw_rectangle(
                flake.x,
                flake.y,
                state.grid_size_px as f32,
                state.grid_size_px as f32,
                color,
            );
        }

        next_frame().await
    }
}
