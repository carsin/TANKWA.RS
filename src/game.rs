use macroquad::prelude::*;
use super::player;

pub const BULLET_WIDTH: f32 = 1.;
pub const BULLET_SPEED: f32 = 10.;

pub struct Game {
    player: player::Player,
    pub projectiles: Vec<Projectile>,
}

pub struct Projectile {
    pub pos: Vec2,
    pub dir: Vec2,
    pub vel: Vec2,
    pub shot_at: f64,
    pub collided: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player: player::Player::new(),
            projectiles: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        let delta = get_frame_time();
        let frame_t = get_time();

        // shoot bullet on click
        // TODO: get position always & move pos to player obj
        if is_mouse_button_down(MouseButton::Left) && frame_t - self.player.last_shot > 0.1 {
            self.player.last_shot = frame_t;
            self.projectiles.push(self.player.shoot());
        }

        // update bullets
        for proj in self.projectiles.iter_mut() {
            // move pos and colliders
            proj.pos += proj.vel;
        }

        self.player.update(delta);
        self.projectiles.retain(|bullet| bullet.shot_at + 1.5 > frame_t && !bullet.collided);
    }

    pub fn render(&self) {
        clear_background(BLACK);

        // projectiles
        for proj in self.projectiles.iter() {
            let pos1 = proj.pos - proj.vel;
            let pos2 = proj.pos - proj.vel * 2.;
            draw_line(pos1.x, pos1.y, pos2.x, pos2.y, BULLET_WIDTH, YELLOW);
        }

        // player
        self.player.render();
    }
}
