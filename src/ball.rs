use crate::block::Vec2D;
use raylib::ffi::Color;
use raylib::prelude::*;

pub struct Ball {
    pub position: Vec2D,
    pub speed: Vec2D,
    pub radius: f32,
    pub color: Color,
    pub fellof: bool,
}

impl Ball {
    pub fn new(position: Vec2D, speed: Vec2D, radius: f32, color: Color) -> Self {
        Self {
            position,
            speed,
            radius,
            color,
            fellof: false,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let center = Vector2::new(self.position.x, self.position.y);
        d.draw_circle_v(center, self.radius, self.color);
    }
    pub fn check_collision(&mut self, rec: Rectangle) -> bool {
        rec.check_collision_circle_rec(Vector2::new(self.position.x, self.position.y), self.radius)
    }
}
