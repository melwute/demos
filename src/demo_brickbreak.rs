use crate::demo_state::DemoState;

use macroquad::prelude::{
    draw_line, draw_rectangle, get_frame_time, is_mouse_button_down, is_mouse_button_pressed,
    is_mouse_button_released, mouse_position, mouse_position_local, screen_height, screen_width,
    vec2, Color, MouseButton, Vec2, RED,
};
use rand::{thread_rng, Rng};

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
    brick_spawn_colors: Vec<Color>,

    screen_size: Vec2,

    ball_spawn_colors: Vec<Color>,
    ball_dimension: Vec2,
    balls: Vec<Ball>,

    seconds_until_next_shot: f32,
    ball_spawn_location: Vec2,
}

impl DemoBrickBreakState {
    pub fn new() -> Self {
        let screen_size = vec2(screen_width(), screen_height());
        let mut rng = rand::thread_rng();

        let ball_spawn_colors = vec![RED];

        let brick_spawn_colors = vec![
            Color::from_rgba(208, 58, 209, 255),
            Color::from_rgba(247, 83, 82, 255),
            Color::from_rgba(253, 128, 20, 255),
            Color::from_rgba(255, 144, 36, 255),
            Color::from_rgba(5, 179, 32, 255),
            Color::from_rgba(109, 101, 246, 255),
        ];

        let mut balls = Vec::new();
        let ball_dimension = vec2(16.0, 16.0);

        for _ in 0..10 {
            let x = rng.gen_range(0.0..screen_size.x - ball_dimension.x);
            let y = rng.gen_range(0.0..screen_size.y - ball_dimension.y);
            let mut velocity = vec2(100.0, 100.0);

            velocity.x = if rng.gen_bool(0.5) {
                -velocity.x
            } else {
                velocity.x
            };

            velocity.y = if rng.gen_bool(0.5) {
                -velocity.y
            } else {
                velocity.y
            };

            balls.push(Ball {
                active: true,
                position: vec2(x, y),
                dimension: ball_dimension,
                velocity,
                color: ball_spawn_colors[rng.gen_range(0..ball_spawn_colors.len())],
            });
        }
        DemoBrickBreakState {
            screen_size,
            balls,
            ball_spawn_location: calc_ball_spawn_location(screen_size),
            ball_dimension,
            seconds_until_next_shot: 0.0,
            brick_spawn_colors: brick_spawn_colors,
            ball_spawn_colors: ball_spawn_colors,
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

            let mut rng = rand::thread_rng();
            self.balls.push(Ball {
                position: self.ball_spawn_location,
                velocity: m,
                dimension: self.ball_dimension,
                active: true,
                color: self.ball_spawn_colors[rng.gen_range(0..self.ball_spawn_colors.len())],
            });

            self.seconds_until_next_shot = 0.25;
        }

        for ball in self.balls.iter_mut() {
            if ball.active {
                move_ball(ball, self.screen_size, seconds_delta);
            }
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
                let color = Color::new(0.0, 1.0, 0.0, 1.0);
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
