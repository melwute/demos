use crate::demo_state::DemoState;

use macroquad::prelude::*;
use std::time::{Duration, Instant};
use std::{
    cmp::min,
    ops::{AddAssign, SubAssign},
};

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

pub struct DemoSomethingTech {
    screen_size: Vec2,

    player: Blip,
    target: Option<u32>, //todo struct for entity id
    last_entity_id: u32,
    others: Vec<Blip>,
}

pub struct CombatStats {
    name: String,

    health: CappedValue,

    min_damage: i32,
    max_damage: i32,
    attack_speed: Duration,
    until_next_attack: Duration,
    chance_to_hit: f32,
}

pub struct Blip {
    entity_id: u32,
    position: Vec2,
    radius: f32,
    combat: CombatStats,
}

pub fn random_blip(screen_size: Vec2, entity_id: u32) -> Blip {
    let x: f32 = rand::gen_range(0.0, screen_size.x) as f32;
    let y: f32 = rand::gen_range(0.0, screen_size.y) as f32;
    Blip {
        entity_id,
        position: vec2(x, y),
        radius: 8.0,
        combat: CombatStats {
            name: "blip".to_string(),
            health: CappedValue::new(10, 10),
            min_damage: 1,
            max_damage: 2,
            attack_speed: Duration::from_secs_f64(1.0),
            until_next_attack: Duration::from_secs_f64(0.0),
            chance_to_hit: 0.9,
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

impl DemoSomethingTech {
    pub async fn new() -> Self {
        let screen_size = vec2(screen_width(), screen_height());

        let blip = Blip {
            entity_id: 0,
            position: vec2(screen_size.x / 2.0, screen_size.y / 2.0),
            radius: 8.0,
            combat: CombatStats {
                name: "Player".to_string(),
                health: CappedValue::new(10, 10),
                min_damage: 1,
                max_damage: 2,
                attack_speed: Duration::from_secs_f64(1.0),
                until_next_attack: Duration::from_secs_f64(0.5),
                chance_to_hit: 0.5,
            },
        };

        let mut demo = DemoSomethingTech {
            screen_size,
            player: blip,
            target: None,
            others: Vec::new(),
            last_entity_id: 0,
        };

        for _ in 0..2 {
            demo.last_entity_id += 1;
            demo.others
                .push(random_blip(screen_size, demo.last_entity_id));
        }

        demo
    }
}

fn draw_blip(blip: &Blip, color: &Color) {
    draw_circle(blip.position.x, blip.position.y, blip.radius, *color);
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

// see screen shot detail.
// yellow cirlces for self
// red circles for active hostiles
// green circles for others? ... or grey or something.

// tab target to switch between targets.
// press one to auto attack.

impl DemoState for DemoSomethingTech {
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
            self.target = None;
        }

        if is_key_released(KeyCode::Tab) {
            println!("boink");
            let targets: Vec<u32> = self.others.iter().map(|x| x.entity_id).collect();
            println!("{:?}", targets);
            if targets.len() > 0 {
                if let Some(current) = self.target {
                    println!("curr entity: {:?}", current);
                    let index = targets.iter().position(|&i| i == current);
                    println!("entity index{:?}", index);
                    if let Some(index) = index {
                        let mut index = index + 1;
                        println!("next index{:?}", index);
                        if index >= targets.len() {
                            index = 0;
                        }
                        self.target = Some(targets[index]);
                    }
                } else {
                    self.target = Some(targets[0]);
                }
            }
        }

        if let Some(target) = self.target {
            let target = self.others.iter_mut().find(|x| x.entity_id == target);
            if let Some(target) = target {
                attack(
                    &mut self.player.combat,
                    &mut target.combat,
                    seconds_duration,
                );
            }
        }

        self.others.retain(|r| r.combat.health.current > 0);

        if let Some(target) = self.target {
            let found_target = self.others.iter_mut().find(|x| x.entity_id == target);
            if let None = found_target {
                self.target = None;
            }
        }

        draw_blip(&self.player, &GREEN);

        for blip in &self.others {
            let color = if let Some(current) = self.target {
                if current == blip.entity_id {
                    RED
                } else {
                    YELLOW
                }
            } else {
                YELLOW
            };

            draw_blip(blip, &color);
        }

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
            &format!("Target {:?}", self.target),
            current.x,
            current.y,
            font_size,
            WHITE,
        );
        current.y += targeting_spacing_y;

        if let Some(target) = self.target {
            let target = self.others.iter_mut().find(|x| x.entity_id == target);
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
    }
}
