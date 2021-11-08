use macroquad::prelude::*;

mod tank;
mod game;
mod projectile;

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
    let tank_texture: Texture2D = load_texture("assets/tonk.png").await.unwrap();
    let textures = vec![tank_texture];
    let mut game = game::Game::new(textures);

    loop {
        if is_key_down(KeyCode::Escape) { break; }
        game.update();
        game.render();
        next_frame().await
    }
}
