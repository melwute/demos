use glam::IVec2;
use macroquad::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Display {
    pub spritesheet_location: Rect,
}

pub struct Entity {
    pub position: IVec2,
    pub display: Display,
}
