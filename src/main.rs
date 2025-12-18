use macroquad::prelude::*;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}

#[macroquad::main("Sprite qui bouge")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    const MOV_SPEED: f32 = 200.0;

    let mut squares: Vec<Shape> = vec![];
    let mut circle = Shape {
        size: 20.0,
        speed: MOV_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };

    loop {
        clear_background(DARKGREEN);

        let dt = get_frame_time();
        if is_key_down(KeyCode::Right) {
            circle.x += MOV_SPEED * dt;
        }
        if is_key_down(KeyCode::Left) {
            circle.x -= MOV_SPEED * dt;
        }
        if is_key_down(KeyCode::Up) {
            circle.y -= MOV_SPEED * dt;
        }
        if is_key_down(KeyCode::Down) {
            circle.y += MOV_SPEED * dt;
        }
        circle.x = clamp(circle.x, 0.0, screen_width());
        circle.y = clamp(circle.y, 0.0, screen_height());

        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size,
                speed: rand::gen_range(50.0, 400.0),
                x: rand::gen_range(0.0, screen_width()),
                y: -size,
            });
        }

        for square in &mut squares {
            square.y += square.speed * dt;
        }
        squares.retain(|square| square.y < screen_height() + square.size);
        draw_circle(circle.x, circle.y, circle.size, YELLOW);

        for square in &squares {
            draw_rectangle(
                square.x - square.size,
                square.y - square.size,
                square.size,
                square.size,
                RED,
            );
        }
        next_frame().await;
    }
}
