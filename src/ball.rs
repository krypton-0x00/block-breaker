use crate::block::Vec2D;
use raylib::ffi::{Color, DrawCircleV};

pub struct Ball {
    pub position: Vec2D,
    pub speed: Vec2D,
    pub radius: f32,
    pub color: Color,
}

impl Ball {
    pub fn new(position: Vec2D, speed: Vec2D, radius: f32, color: Color) -> Self {
        Self {
            position,
            speed,
            radius,
            color,
        }
    }
    pub fn draw(&self) {
        unsafe {
            DrawCircleV(
                raylib::ffi::Vector2 {
                    x: self.position.x,
                    y: self.position.y,
                },
                self.radius,
                self.color,
            );
        }
    }
}
