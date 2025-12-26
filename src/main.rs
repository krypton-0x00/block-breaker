//TODO: Add game state to handle game state bugs like (game lost after won etc..)
use raylib::prelude::*;
use std::process::exit;
mod ball;
mod block;
use ball::Ball;
use block::{Block, Vec2D};

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Block-Breaker").build();

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

    let screen_height = rl.get_screen_height();

    let ball_x = paddle_x;
    let ball_y = (rl.get_screen_height() - 50) as f32;
    let ball_position = Vec2D::new(ball_x, ball_y);
    let ball_speed = Vec2D::new(300.0, -300.0);
    let ball_radius = 5.0;
    let ball_color = Color::WHITE.into();
    let mut ball = Ball::new(ball_position, ball_speed, ball_radius, ball_color);

    let mut bricks = Vec::with_capacity(ROWS * COLS);

    const BRICK_WIDTH: f32 = 70.0;
    const BRICK_HEIGHT: f32 = 30.0;
    const BRICK_GAP: f32 = 5.0;

    const START_X: f32 = 22.0;
    const START_Y: f32 = 10.0;

    const COLS: usize = 8;
    const ROWS: usize = 6;

    for row in 0..ROWS {
        for col in 0..COLS {
            let x = START_X + col as f32 * (BRICK_WIDTH + BRICK_GAP);
            let y = START_Y + row as f32 * (BRICK_HEIGHT + BRICK_GAP);

            let color = match row {
                0 => Color::YELLOW,
                1 => Color::PINK,
                2 => Color::GREEN,
                4 => Color::BLUE,
                5 => Color::PURPLE,
                6 => Color::RED,
                7 => Color::CYAN,
                _ => Color::WHITESMOKE,
            };

            bricks.push(Block::new(
                Vec2D::new(x, y),
                BRICK_HEIGHT,
                BRICK_WIDTH,
                0.0,
                color.into(),
            ));
        }
    }
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

        //collision bw ball and paddle
        if ball.check_collision(paddle.get_rect().into()) {
            if ball.speed.y < 0.0 {
                ball.speed.y *= 1.0;
                ball.speed.x *= -1.0;
            } else {
                ball.speed.y *= -1.0;
                ball.speed.x *= 1.0;
            }
        }
        //collision of ball with bricks
        for item in &mut bricks {
            if item.check_collision(&mut ball) {
                // println!("Collided {}", item.position.x)
                item.is_broken = true;
            }
        }

        //DEBUG
        if rl.is_key_down(KeyboardKey::KEY_BACKSPACE) {
            bricks.clear();
        }

        //if fell of
        if ball.position.y + ball.radius >= rl.get_screen_height() as f32 {
            ball.position.y = rl.get_screen_height() as f32 + ball.radius;
            ball.fellof = true;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        if bricks.is_empty() {
            d.draw_text("You Won", 210, screen_height / 2 - 30, 50, Color::GREEN);
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

        bricks.iter().for_each(|item| {
            if !item.is_broken {
                item.draw(&mut d);
            }
        });

        paddle.draw(&mut d);
        ball.draw(&mut d);
    }
}
