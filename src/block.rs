use raylib::{
    RaylibHandle,
    ffi::{Color, KeyboardKey, Rectangle},
    prelude::{RaylibDraw, RaylibDrawHandle},
};

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
    pub speed: f32,
}

impl Block {
    pub fn new(position: Vec2D, height: f32, width: f32, speed: f32, color: Color) -> Self {
        Self {
            position,
            height,
            width,
            color,
            is_broken: false,
            speed,
        }
    }
    pub fn get_rect(&self) -> Rectangle {
        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: self.width,
            height: self.height,
        }
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let block = Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: self.width,
            height: self.height,
        };

        d.draw_rectangle_rec(block, self.color);
    }
    pub fn movement(&mut self, rl: &mut RaylibHandle) {
        let deltatime = rl.get_frame_time();

        if rl.is_key_down(KeyboardKey::KEY_D) | rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            if self.position.x < rl.get_screen_width() as f32 - self.width as f32 {
                self.position.x += self.speed * deltatime;
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_A) | rl.is_key_down(KeyboardKey::KEY_LEFT) {
            if self.position.x > 0 as f32 {
                self.position.x -= self.speed * deltatime;
            }
        }
    }
}
