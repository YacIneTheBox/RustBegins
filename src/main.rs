use macroquad::prelude::*;

#[macroquad::main("Sprite qui bouge")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    let speed = 200.0;

    loop {
        clear_background(BLACK);

        // gestion des touche
        let dt = get_frame_time();
        if is_key_down(KeyCode::Right) {
            x += speed * dt
        }
        if is_key_down(KeyCode::Left) {
            x -= speed * dt
        }
        if is_key_down(KeyCode::Up) {
            y -= speed * dt
        }
        if is_key_down(KeyCode::Down) {
            y += speed * dt
        }

        draw_circle(x, y, 20.0, YELLOW);

        draw_text("Fleche", 10.0, 10.0, 20.0, WHITE);

        next_frame().await;
    }
}
