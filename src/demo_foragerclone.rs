use crate::demo_state::DemoState;

use macroquad::prelude::*;
use std::time::{Duration, Instant};
use std::{
    cmp::min,
    ops::{AddAssign, SubAssign},
};

const TILE_WIDTH: f32 = 16.0;
const TILE_HEIGHT: f32 = 16.0;

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

pub struct CappedValue {
    current: i32,
    max: i32,
}

impl AddAssign<i32> for CappedValue {
    fn add_assign(&mut self, other: i32) {
        let mut desired = self.current + other;

        if desired >= self.max {
            desired = self.max;
        }

        *self = Self {
            current: desired,
            max: self.max,
        };
    }
}

impl SubAssign<i32> for CappedValue {
    fn sub_assign(&mut self, other: i32) {
        let mut desired = self.current - other;

        if desired <= 0 {
            desired = 0;
        }

        *self = Self {
            current: desired,
            max: self.max,
        };
    }
}

impl CappedValue {
    fn new(current: i32, max: i32) -> Self {
        CappedValue { current, max }
    }

    fn with_max(max: i32) -> Self {
        Self::new(0, max)
    }
}

pub struct DemoForagerClone {
    screen_size: Vec2,

    sprite_sheet: Texture2D,

    player: Entity,

    last_entity_id: u32,
    trees: Vec<Entity>,
}

pub struct CombatStats {
    name: String,

    health: CappedValue,

    min_damage: i32,
    max_damage: i32,
    attack_speed: Duration,
    until_next_attack: Duration,
    chance_to_hit: f32,
    auto_attacking: bool,
    target: Option<u32>, //todo struct for entity id
}

impl CombatStats {
    pub fn clear_target(&mut self) {
        self.auto_attacking = false;
        self.target = None;
    }
}

#[derive(Debug)]
pub enum EntityState {
    Idle,
    Moving { location: Vec2 },
    PerformingAction,
}

#[derive(Debug, Copy, Clone)]
pub enum EntityAction {
    Nothing,
    MovingTowards { location: Vec3A },
}

pub struct Entity {
    entity_id: u32,
    position: Vec2,
    dimension: Vec2,
    spritesheet_location: Rect,
    combat: CombatStats,

    state: EntityState,
    action: EntityAction,
}

impl Entity {
    fn move_towards(&mut self, target: Vec2, move_speed: f32, delta: f32) -> bool {
        let direction = target - self.position;
        let step_size = move_speed * delta;
        self.position += direction.normalize() * step_size;
        let at_target = direction.length() < step_size;
        return at_target;
    }
}

pub fn random_entity(screen_size: Vec2, entity_id: u32) -> Entity {
    let x: f32 = rand::gen_range(0.0, screen_size.x) as f32;
    let y: f32 = rand::gen_range(0.0, screen_size.y) as f32;
    Entity {
        entity_id,
        action: EntityAction::Nothing,
        state: EntityState::Idle,
        position: vec2(x, y),
        dimension: vec2(TILE_WIDTH * 2.0, TILE_HEIGHT * 2.0),
        spritesheet_location: SPRITESHEET_FARMER,
        combat: CombatStats {
            name: "Farmer".to_string(),
            health: CappedValue::new(10, 10),
            min_damage: 1,
            max_damage: 2,
            attack_speed: Duration::from_secs_f64(1.0),
            until_next_attack: Duration::from_secs_f64(0.0),
            chance_to_hit: 0.5,
            auto_attacking: false,
            target: None,
        },
    }
}

fn attack(source: &mut CombatStats, target: &mut CombatStats, delta: Duration) {
    if delta < source.until_next_attack {
        source.until_next_attack -= delta;
        return;
    }

    let roll = rand::gen_range(0.0, 1.0);
    if roll < source.chance_to_hit {
        let dmg = rand::gen_range(source.min_damage, source.max_damage + 1);
        target.health -= dmg;
        println!("{} hit {} for {}", source.name, target.name, dmg);
    } else {
        println!("{} misses! roll {}", source.name, roll);
    }

    let leftover = delta - source.until_next_attack;
    assert!(leftover < source.attack_speed);
    source.until_next_attack = source.attack_speed - leftover;
}

impl DemoForagerClone {
    pub async fn new() -> Self {
        let screen_size = vec2(screen_width(), screen_height());

        let player = Entity {
            entity_id: 0,
            action: EntityAction::Nothing,
            state: EntityState::Idle,
            position: vec2(screen_size.x / 2.0, screen_size.y / 2.0),
            spritesheet_location: SPRITESHEET_WARRIOR,
            dimension: vec2(TILE_WIDTH * 2.0, TILE_HEIGHT * 2.0),
            combat: CombatStats {
                name: "Player".to_string(),
                health: CappedValue::new(10, 10),
                min_damage: 1,
                max_damage: 2,
                attack_speed: Duration::from_secs_f64(1.0),
                until_next_attack: Duration::from_secs_f64(0.5),
                chance_to_hit: 0.9,
                auto_attacking: false,
                target: None,
            },
        };

        let sprite_sheet = load_texture("src/colored_transparent_packed.png")
            .await
            .unwrap();
        sprite_sheet.set_filter(FilterMode::Nearest);

        let mut demo = DemoForagerClone {
            screen_size,
            sprite_sheet,
            player,
            trees: Vec::new(),
            last_entity_id: 0,
        };

        for _ in 0..2 {
            demo.last_entity_id += 1;
            demo.trees
                .push(random_entity(screen_size, demo.last_entity_id));
        }

        demo
    }
}

fn draw_sprite(tileset: &Texture2D, position: Vec2, dimension: Vec2, spritesheet_location: Rect) {
    let mut params = DrawTextureParams::default();
    params.source = Some(spritesheet_location);
    params.dest_size = Some(dimension);
    draw_texture_ex(*tileset, position.x, position.y, WHITE, params);
}

fn draw_entity(tileset: &Texture2D, entity: &Entity) {
    draw_sprite(
        tileset,
        entity.position,
        entity.dimension,
        entity.spritesheet_location,
    );

    let health_offset_y = -6.0;
    let name_offset_y = health_offset_y - 6.0;

    draw_rectangle(
        entity.position.x,
        entity.position.y + health_offset_y,
        entity.dimension.x, //vary size be health %tages
        6.0,
        RED,
    );
    let health_remaining = entity.combat.health.current as f32 / entity.combat.health.max as f32;
    draw_rectangle(
        entity.position.x,
        entity.position.y + health_offset_y,
        entity.dimension.x * health_remaining,
        6.0,
        YELLOW,
    );

    //draw_circle(mouse_x, mouse_y, 3.0, YELLOW);
    draw_text(
        &entity.combat.name,
        entity.position.x,
        entity.position.y + name_offset_y,
        20.0,
        DARKGRAY,
    );
}
fn get_player_movement() -> Vec2 {
    let mut vel = Vec2::default();
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        vel.x += 1.0;
    }
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        vel.x -= 1.0;
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        vel.y += 1.0;
    }
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        vel.y -= 1.0;
    }

    return vel;
}

impl DemoState for DemoForagerClone {
    fn process_frame(&mut self) {
        let seconds_delta = get_frame_time();
        let seconds_duration = Duration::from_secs_f32(seconds_delta);
        let current_screen = vec2(screen_width(), screen_height());
        if current_screen != self.screen_size {
            println!("resize {:?} => {:?}", self.screen_size, current_screen);
            self.screen_size = current_screen;
        }

        let vel = get_player_movement();
        self.player.position += vel * 200.0 * seconds_delta;

        if is_key_released(KeyCode::GraveAccent) {
            self.player.combat.target = None;
        }

        if is_key_released(KeyCode::Key1) {
            if let Some(current) = self.player.combat.target {
                self.player.combat.auto_attacking = true;
            }
        }

        let move_speed = 100.0;
        match self.player.state {
            EntityState::Idle => {}
            EntityState::Moving { location } => {
                let at_target = self
                    .player
                    .move_towards(location, move_speed, seconds_delta);
                if at_target {
                    self.player.state = EntityState::Idle;
                }
            }
            EntityState::PerformingAction => {}
        }

        let mouse = mouse_position();

        if is_mouse_button_down(MouseButton::Left) {
            draw_text("down", 150.0, 30.0, 32.0, WHITE);
            self.player.state = EntityState::Moving {
                location: Vec2::new(mouse.0, mouse.1),
            };
        }

        let max_cursor_dist = 4000000.0;
        let max_player_dist = 2000500.0;

        let mut closest_dist = f32::MAX;
        let mut closest_entity = None;
        for trees in self.trees.iter() {
            let tree_mid = (trees.dimension / 2.0) + trees.position;
            let player_mid = (self.player.dimension / 2.0) + self.player.position;

            let player_dist =
                (tree_mid.x - player_mid.x).powf(2.0) + (tree_mid.y - player_mid.y).powf(2.0);
            let mouse_dist = (tree_mid.x - mouse.0).powf(2.0) + (tree_mid.y - mouse.1).powf(2.0);

            if mouse_dist < closest_dist
                && player_dist <= max_player_dist
                && mouse_dist <= max_cursor_dist
            {
                closest_dist = mouse_dist;
                closest_entity = Some(trees.entity_id);
            }
        }
        self.player.combat.target = closest_entity;

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(target) = self.player.combat.target {
                let target = self.trees.iter_mut().find(|x| x.entity_id == target);
                if let Some(target) = target {
                    attack(
                        &mut self.player.combat,
                        &mut target.combat,
                        seconds_duration,
                    );
                }
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            //determine the target
            //let target_to_mouse = None;
            let mouse = mouse_position();
            let mut closest_dist = f32::MAX;
            let mut closest_entity = None;
            for trees in self.trees.iter() {
                let mid = (trees.dimension / 2.0) + trees.position;
                let dist = (mid.x - mouse.0).powf(2.0) + (mid.y - mouse.1).powf(2.0);
                if dist < closest_dist {
                    closest_dist = dist;
                    closest_entity = Some(trees.entity_id);
                }
            }

            println!(
                "closest entity is {:?} at {:?}",
                closest_entity, closest_dist
            );
        }

        if is_key_released(KeyCode::Tab) {
            println!("boink");
            let targets: Vec<u32> = self.trees.iter().map(|x| x.entity_id).collect();
            println!("{:?}", targets);
            if targets.len() > 0 {
                if let Some(current) = self.player.combat.target {
                    println!("curr entity: {:?}", current);
                    let index = targets.iter().position(|&i| i == current);
                    println!("entity index{:?}", index);
                    if let Some(index) = index {
                        let mut index = index + 1;
                        println!("next index{:?}", index);
                        if index >= targets.len() {
                            index = 0;
                        }
                        self.player.combat.target = Some(targets[index]);
                    }
                } else {
                    self.player.combat.target = Some(targets[0]);
                }
            }
        }

        if let Some(target) = self.player.combat.target {
            let target = self.trees.iter_mut().find(|x| x.entity_id == target);
            if let Some(target) = target {
                if self.player.combat.auto_attacking {
                    attack(
                        &mut self.player.combat,
                        &mut target.combat,
                        seconds_duration,
                    );
                }
            }
        }

        self.trees.retain(|r| r.combat.health.current > 0);

        if let Some(target) = self.player.combat.target {
            let found_target = self.trees.iter_mut().find(|x| x.entity_id == target);
            if let None = found_target {
                self.player.combat.clear_target();
            }
        }

        draw_entity(&self.sprite_sheet, &self.player);

        for entity in &self.trees {
            if let Some(current) = self.player.combat.target {
                if current == entity.entity_id {
                    draw_sprite(
                        &self.sprite_sheet,
                        entity.position,
                        entity.dimension,
                        SPRITESHEET_TARGET,
                    );
                }
            };

            draw_entity(&self.sprite_sheet, entity);
        }

        self.draw_ui();
    }
}

impl DemoForagerClone {
    fn draw_ui(&mut self) {
        //ui
        let targeting_root = vec2(10.0, 20.0);
        let font_size = 16.0;
        let targeting_spacing_y = 16.0;
        let mut current = targeting_root.clone();
        draw_text("Self", current.x, current.y, font_size, WHITE);
        current.y += targeting_spacing_y;
        draw_text(
            &format!(
                "Health: ({}/{})",
                self.player.combat.health.current, self.player.combat.health.max
            ),
            current.x,
            current.y,
            font_size,
            WHITE,
        );
        current.y += targeting_spacing_y;
        draw_text(
            &format!("Target {:?}", self.player.combat.target),
            current.x,
            current.y,
            font_size,
            WHITE,
        );
        current.y += targeting_spacing_y;
        if let Some(target) = self.player.combat.target {
            let target = self.trees.iter_mut().find(|x| x.entity_id == target);
            if let Some(target) = target {
                draw_text(
                    &format!(
                        "Health: ({}/{})",
                        target.combat.health.current, target.combat.health.max
                    ),
                    current.x,
                    current.y,
                    font_size,
                    WHITE,
                );
            }
        }
        current.y += targeting_spacing_y;

        draw_text(&format!("Skills"), current.x, current.y, font_size, WHITE);
        current.y += targeting_spacing_y;

        draw_text(
            &format!("1) Auto attack"),
            current.x,
            current.y,
            font_size,
            WHITE,
        );
        current.y += targeting_spacing_y;
    }
}
