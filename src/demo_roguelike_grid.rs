use crate::demo_state::DemoState;

use macroquad::prelude::*;
use std::time::{Duration, Instant};
use std::{
    cmp::min,
    ops::{AddAssign, SubAssign},
};

// https://www.gridbugs.org/roguelike-tutorial-2020/
const TILE_WIDTH: f32 = 16.0;
const TILE_HEIGHT: f32 = 16.0;

pub struct DemoRoguelikeGrid {
    screen_size: IVec2, //TODO: Dimension type?
    grid_size: IVec2,   //TODO: Dimension type?
    font: Font,

    player_location: IVec2,
}

impl DemoRoguelikeGrid {
    pub async fn new() -> Self {
        let screen_results = calc_screen_size();

        let font = load_ttf_font_from_bytes(include_bytes!("fonts/PxPlus_IBM_CGA.ttf")).unwrap();

        let player_location = screen_results.grid_size / 2;

        let mut demo = DemoRoguelikeGrid {
            font,
            screen_size: screen_results.screen_size,
            grid_size: screen_results.grid_size,
            player_location,
        };

        demo
    }
}

pub struct ScreenRefreshResult {
    screen_size: IVec2,
    grid_size: IVec2,
}
fn calc_screen_size() -> ScreenRefreshResult {
    let current_screen = vec2(screen_width(), screen_height());

    let screen_size = IVec2::new(screen_width() as i32, screen_height() as i32);
    let grid_size = IVec2::new(
        screen_size.x / TILE_WIDTH as i32,
        screen_size.y / TILE_HEIGHT as i32,
    );
    ScreenRefreshResult {
        screen_size,
        grid_size,
    }
}

fn to_screen_px(grid: IVec2) -> Vec2 {
    Vec2::new(grid.x as f32 * TILE_WIDTH, grid.y as f32 * TILE_HEIGHT)
}

impl DemoRoguelikeGrid {
    pub fn try_move_player(&mut self, delta: IVec2) {
        let new = self.player_location + delta;
        //check valid later
        self.player_location = new;
    }
}

//TODO: cardinal directions would make more sense type wise.
fn get_player_movement() -> IVec2 {
    let mut delta = IVec2::default();
    if is_key_released(KeyCode::Right) || is_key_released(KeyCode::D) {
        delta.x += 1;
    }
    if is_key_released(KeyCode::Left) || is_key_released(KeyCode::A) {
        delta.x -= 1;
    }
    if is_key_released(KeyCode::Down) || is_key_released(KeyCode::S) {
        delta.y += 1;
    }
    if is_key_released(KeyCode::Up) || is_key_released(KeyCode::W) {
        delta.y -= 1;
    }
    return delta;
}

impl DemoState for DemoRoguelikeGrid {
    fn process_frame(&mut self) {
        let seconds_delta = get_frame_time();
        let seconds_duration = Duration::from_secs_f32(seconds_delta);

        /*
         //TODO  handle this mess when we have proper camera support.
        let current_screen = vec2(screen_width(), screen_height());
        if current_screen != self.screen_size {
            let screen_results = calc_screen_size();
            println!("resize {:?} => {:?}", self.screen_size, current_screen);
            self.screen_size = current_screen;
        }
         */

        let player_delta = get_player_movement();
        self.try_move_player(player_delta);

        //render grid.
        //TODO need to handle when the screen size isn't a multiple of the tile size.
        let mut params = TextParams::default();
        params.font = self.font;
        params.font_size = TILE_WIDTH as u16;
        params.color = WHITE;

        let screen = to_screen_px(self.player_location);
        draw_text_ex("@", screen.x, screen.y, params);
    }
}
