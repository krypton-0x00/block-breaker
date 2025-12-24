use raylib::ffi::{Color, DrawRectangle, DrawRectangleRec, Rectangle};

pub struct Vec2D {
    pub x: f32,
    pub y: f32,
}

impl Vec2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub struct Block {
    pub position: Vec2D,
    pub height: f32,
    pub width: f32,
    pub color: Color,
    pub is_broken: bool,
}

impl Block {
    pub fn new(position: Vec2D, height: f32, width: f32, color: Color) -> Self {
        Self {
            position,
            height,
            width,
            color,
            is_broken: false,
        }
    }
    pub fn draw(&self) {
        let block = Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: self.width,
            height: self.height,
        };

        unsafe {
            DrawRectangleRec(block, self.color);
        }
    }
}
