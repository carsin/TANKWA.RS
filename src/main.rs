use macroquad::prelude::*;

mod player;
mod game;

#[macroquad::main("Asteroids --yabai")]
async fn main() {
    let mut game = game::Game::new();

    loop {
        if is_key_down(KeyCode::Q) {
            break;
        }
        game.update();
        game.render();
        next_frame().await
    }
}
