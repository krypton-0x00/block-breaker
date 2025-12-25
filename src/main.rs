use std::{process::exit, thread::sleep, time::Duration};

use raylib::prelude::*;
mod ball;
mod block;
use ball::Ball;
use block::{Block, Vec2D};

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    let paddle_x = (rl.get_screen_width() / 2) as f32;
    let paddle_y = (rl.get_screen_height() - 50) as f32;
    let paddle_position = Vec2D::new(paddle_x, paddle_y);
    let paddle_speed = 400.0;
    let mut paddle = Block::new(
        paddle_position,
        15.0,
        100.0,
        paddle_speed,
        Color::PINK.into(),
    );

    let screen_width = rl.get_screen_width();
    let screen_height = rl.get_screen_height();

    let ball_x = paddle_x;
    let ball_y = (rl.get_screen_height() - 50) as f32;
    let ball_position = Vec2D::new(ball_x, ball_y);
    let ball_speed = Vec2D::new(300.0, -300.0);
    let ball_radius = 5.0;
    let ball_color = Color::WHITE.into();
    let mut ball = Ball::new(ball_position, ball_speed, ball_radius, ball_color);

    while !rl.window_should_close() {
        let deltatime = rl.get_frame_time();
        paddle.movement(&mut rl);

        ball.position.x += ball.speed.x * deltatime;
        ball.position.y += ball.speed.y * deltatime;

        //left
        if ball.position.x + ball.radius >= rl.get_screen_width() as f32 {
            ball.position.x = (rl.get_screen_width() as f32) - ball.radius;
            ball.speed.x *= -1 as f32;
        }
        //top
        if ball.position.y + ball.radius <= 0 as f32 {
            ball.position.y = (0 as f32) + ball.radius;
            ball.speed.y *= -1 as f32;
        }
        //right
        if ball.position.x + ball.radius <= 0.0 {
            ball.position.x = 0.0 + ball.radius;
            ball.speed.x *= -1 as f32;
        }

        //collision
        if ball.check_collision(paddle.get_rect().into()) {
            if ball.speed.y < 0.0 {
                ball.speed.y *= 1.0;
            } else {
                ball.speed.y *= -1.0;
            }
        }
        //if fell of
        if ball.position.y + ball.radius >= rl.get_screen_height() as f32 {
            ball.position.y = rl.get_screen_height() as f32 + ball.radius;
            ball.fellof = true;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        if ball.fellof {
            d.draw_text("You Lost!", 210, screen_height / 2 - 30, 50, Color::RED);
            d.draw_text(
                "Press ESC to exit",
                210,
                screen_height / 2 + 30,
                25,
                Color::YELLOW,
            );

            if d.is_key_down(KeyboardKey::KEY_ESCAPE) {
                exit(0);
            }
        }

        paddle.draw(&mut d);
        ball.draw(&mut d);
    }
}
