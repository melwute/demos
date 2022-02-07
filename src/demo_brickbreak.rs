use crate::demo_state::DemoState;

use macroquad::prelude::*;

pub struct Brick {
    color: Color,
    position: Vec2,
    dimension: Vec2,
}

pub struct Ball {
    active: bool,
    position: Vec2,
    dimension: Vec2,
    velocity: Vec2,
    color: Color,
}

pub struct DemoBrickBreakState {
    screen_size: Vec2,

    ball_spawn_colors: Vec<Color>,
    ball_dimension: Vec2,
    balls: Vec<Ball>,

    brick_spawn_colors: Vec<Color>,
    bricks: Vec<Brick>,

    seconds_until_next_shot: f32,
    ball_spawn_location: Vec2,
}

impl DemoBrickBreakState {
    pub fn new() -> Self {
        let screen_size = vec2(screen_width(), screen_height());
        let ball_spawn_colors = vec![RED];

        let brick_spawn_colors = vec![
            Color::from_rgba(208, 58, 209, 255),
            Color::from_rgba(247, 83, 82, 255),
            Color::from_rgba(253, 128, 20, 255),
            Color::from_rgba(255, 144, 36, 255),
            Color::from_rgba(5, 179, 32, 255),
            Color::from_rgba(109, 101, 246, 255),
        ];

        let balls = Vec::new();
        let ball_dimension = vec2(16.0, 16.0);

        //
        let mut bricks = Vec::new();
        let rows = 5;
        let columns = 10;
        let brick_height = 22.0;

        for r in 0..rows {
            let width = screen_size.x / columns as f32;
            for c in 0..columns {
                bricks.push(Brick {
                    color: brick_spawn_colors[rand::gen_range(0, brick_spawn_colors.len())],
                    position: vec2(c as f32 * width, r as f32 * brick_height),
                    dimension: vec2(width, brick_height),
                });
            }
        }

        DemoBrickBreakState {
            screen_size,
            balls,
            ball_spawn_location: calc_ball_spawn_location(screen_size),
            ball_dimension,
            seconds_until_next_shot: 0.0,
            brick_spawn_colors: brick_spawn_colors,
            ball_spawn_colors: ball_spawn_colors,
            bricks,
        }
    }
}
pub fn calc_ball_spawn_location(screen_size: Vec2) -> Vec2 {
    let distance_from_bottom = 18.0;

    vec2(screen_size.x / 2.0, screen_size.y - distance_from_bottom)
}

fn move_ball(ball: &mut Ball, screen_size: Vec2, seconds_delta: f32) {
    ball.position += ball.velocity * seconds_delta;

    if ball.position.x <= 0.0 {
        ball.position.x = 0.0;
        ball.velocity.x *= -1.0;
    }
    if ball.position.x >= screen_size.x - ball.dimension.x {
        ball.position.x = screen_size.x - ball.dimension.x;
        ball.velocity.x *= -1.0;
    }

    if ball.position.y <= 0.0 {
        ball.position.y = 0.0;
        ball.velocity.y *= -1.0;
    }

    if ball.position.y >= screen_size.y - ball.dimension.y {
        ball.position.y = screen_size.y - ball.dimension.y;
        ball.velocity.y *= -1.0;
        ball.active = false;
    }
}

impl DemoState for DemoBrickBreakState {
    fn process_frame(&mut self) {
        let seconds_delta = get_frame_time();
        let current_screen = vec2(screen_width(), screen_height());
        if current_screen != self.screen_size {
            println!("resize {:?} => {:?}", self.screen_size, current_screen);
            self.screen_size = current_screen;
            self.ball_spawn_location = calc_ball_spawn_location(current_screen);
        }

        let (mouse_x, mouse_y) = mouse_position();
        let mouse_pos = vec2(mouse_x, mouse_y);

        self.seconds_until_next_shot -= seconds_delta; //TODO clamp this to 0.0

        if is_mouse_button_down(MouseButton::Left) && self.seconds_until_next_shot <= 0.0 {
            let ball_speed = 325.0;

            let mut m = mouse_pos - self.ball_spawn_location;
            m = m.normalize();
            m *= ball_speed;

            self.balls.push(Ball {
                position: self.ball_spawn_location,
                velocity: m,
                dimension: self.ball_dimension,
                active: true,
                color: self.ball_spawn_colors[rand::gen_range(0, self.ball_spawn_colors.len())],
            });

            self.seconds_until_next_shot = 0.25;
        }

        for ball in self.balls.iter_mut() {
            if ball.active {
                move_ball(ball, self.screen_size, seconds_delta);
            }
        }

        for brick in self.bricks.iter() {
            draw_rectangle(
                brick.position.x,
                brick.position.y,
                brick.dimension.x,
                brick.dimension.y,
                brick.color,
            );
        }

        let mut m = mouse_pos - self.ball_spawn_location;
        m = m.normalize();
        m *= 50.0; //length of line
        m += self.ball_spawn_location;
        draw_line(
            self.ball_spawn_location.x,
            self.ball_spawn_location.y,
            m.x,
            m.y,
            6.0,
            RED,
        );

        for ball in self.balls.iter() {
            if ball.active {
                draw_rectangle(
                    ball.position.x,
                    ball.position.y,
                    ball.dimension.x,
                    ball.dimension.y,
                    ball.color,
                );
            }
        }

        self.balls.retain(|ball| ball.active);
    }
}