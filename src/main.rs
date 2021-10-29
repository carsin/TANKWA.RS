use macroquad::prelude::*;

mod player;
mod game;

fn window_conf() -> Conf {
    Conf {
        window_title: "SHOOTER --yabai".to_owned(),
        window_width: 1600,
        window_height: 900,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
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
