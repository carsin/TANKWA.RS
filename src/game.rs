use macroquad::prelude::*;
use super::player;

pub struct Game {
    player: player::Player,
    pub bullets: Vec<Bullet>,
}

pub struct Bullet {
    pub pos: Vec2,
    vel: Vec2,
    shot_at: f64,
    collided: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player: player::Player::new(),
            bullets: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        let mut last_shot = get_time();
        let frame_t = get_time();

        // move player
        if is_key_down(KeyCode::W) {
            self.player.vel.y -= 0.5;
        } else if is_key_down(KeyCode::S) {
            self.player.vel.y += 0.5;
        }

        if is_key_down(KeyCode::A) {
            self.player.vel.x -= 0.5;
        } else if is_key_down(KeyCode::D) {
            self.player.vel.x += 0.5;
        }

        // shoot bullet on click
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let weiupahasduifgh = mouse_pos.0 / mouse_pos.1;
            let direction = weiupahasduifgh.atan();
            let rotation = direction.to_radians();
            let rot_vec = Vec2::new(rotation.sin(), -rotation.cos());
            self.bullets.push(Bullet {
                pos: self.player.pos + rot_vec,
                vel: rot_vec * 7.,
                shot_at: frame_t,
                collided: false,
            });
            last_shot = frame_t;
        }

        // update bullets
        for bullet in self.bullets.iter_mut() {
            bullet.pos += bullet.vel;
        }

        self.player.update();
        self.bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t);
        self.bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t && !bullet.collided);
    }

    pub fn render(&self) {
        clear_background(LIGHTGRAY);
        // bullets
        for bullet in self.bullets.iter() {
            draw_circle(bullet.pos.x, bullet.pos.y, 2., BLACK);
        }
        // player
        self.player.render();
    }

}
