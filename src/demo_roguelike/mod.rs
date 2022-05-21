use crate::demo_roguelike::entity::*;
use crate::demo_roguelike::map::*;
use crate::demo_state::DemoState;

use macroquad::prelude::*;
use std::time::{Duration, Instant};
use std::{
    cmp::min,
    ops::{AddAssign, SubAssign},
};

mod entity;
mod map;

const TILE_WIDTH: f32 = 16.0;
const TILE_HEIGHT: f32 = 16.0;

const SPRITESHEET_GRASS: Rect = spritesheet_rect(6, 0);
const SPRITESHEET_WALL: Rect = spritesheet_rect(11, 25);
const SPRITESHEET_FARMER: Rect = spritesheet_rect(25, 1);
const SPRITESHEET_TARGET: Rect = spritesheet_rect(36, 12);
const SPRITESHEET_WARRIOR: Rect = spritesheet_rect(31, 0);

const fn spritesheet_rect(location_x: usize, location_y: usize) -> Rect {
    Rect {
        x: (location_x * TILE_WIDTH as usize) as f32,
        y: (location_y * TILE_HEIGHT as usize) as f32,
        w: TILE_WIDTH,
        h: TILE_HEIGHT,
    }
}

pub struct DemoRoguelike {
    screen_size: IVec2, //TODO: Dimension type?
    grid_size: IVec2,   //TODO: Dimension type?
    font: Font,
    sprite_sheet: Texture2D,

    player: Entity,
    map: Map,
}

impl DemoRoguelike {
    pub async fn new() -> Self {
        let screen_results = calc_screen_size();

        let font = load_ttf_font_from_bytes(include_bytes!("fonts/PxPlus_IBM_CGA.ttf")).unwrap();
        let sprite_sheet = load_texture("src/colored_transparent_packed.png")
            .await
            .unwrap();
        sprite_sheet.set_filter(FilterMode::Nearest);

        let player_location = screen_results.grid_size / 2;

        let mut demo = DemoRoguelike {
            font,
            sprite_sheet,
            screen_size: screen_results.screen_size,
            grid_size: screen_results.grid_size,
            map: Map::new(80, 10),
            player: Entity {
                position: player_location,
                display: Display {
                    spritesheet_location: SPRITESHEET_WARRIOR,
                },
            },
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

fn make_wall_at(position: IVec2) -> Entity {
    Entity {
        position: position,
        display: Display {
            spritesheet_location: SPRITESHEET_WALL,
        },
    }
}

fn make_ground_at(position: IVec2) -> Entity {
    Entity {
        position: position,
        display: Display {
            spritesheet_location: SPRITESHEET_GRASS,
        },
    }
}

impl DemoRoguelike {
    pub fn try_move_player(&mut self, delta: IVec2) {
        let new = self.player.position + delta;
        self.player.position = new;
    }
}

fn draw_sprite(tileset: &Texture2D, position: Vec2, spritesheet_location: Rect) {
    let mut params = DrawTextureParams::default();
    params.source = Some(spritesheet_location);
    params.dest_size = Some(Vec2::new(TILE_WIDTH, TILE_HEIGHT));
    draw_texture_ex(*tileset, position.x, position.y, WHITE, params);
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

impl DemoState for DemoRoguelike {
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

        for x in 0..self.map.width() {
            for y in 0..self.map.height() {
                let screen = to_screen_px(IVec2::new(x, y));
                /*
                draw_rectangle(
                    screen.x,
                    screen.y,
                    TILE_WIDTH,
                    TILE_HEIGHT,
                    Color::from_rgba(50, 50, 150, 255),
                );
                 */
                params.color = WHITE;
                draw_sprite(&self.sprite_sheet, screen, SPRITESHEET_GRASS);
                //draw_text_ex(".", screen.x, screen.y, params);
            }
        }
        let screen = to_screen_px(self.player.position);
        /*
        draw_sprite(
            &self.sprite_sheet,
            screen,
            self.player.display.spritesheet_location,
        );
         */
        draw_text_ex("@", screen.x, screen.y, params);
    }
}
