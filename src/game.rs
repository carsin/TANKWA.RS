use macroquad::prelude::*;
use super::player;

pub struct Game {
    player: player::Player,
    pub bullets: Vec<Bullet>,
}

pub struct Bullet {
    pub pos: Vec2,
    pub dir: Vec2,
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
        let delta = get_frame_time();
        let frame_t = get_time();
        // shoot bullet on click
        // TODO: get position always & move pos to player obj
        if is_mouse_button_down(MouseButton::Left) && frame_t - self.player.last_shot > 0.1 {
            let mouse_pos = mouse_position();
            let direction = (vec2(mouse_pos.0, mouse_pos.1) - self.player.pos).normalize();
            self.bullets.push(Bullet {
                dir: direction,
                pos: ((self.player.pos) + direction),
                vel: 10. * direction,
                shot_at: frame_t,
                collided: false,
            });
            self.player.last_shot = frame_t;
        }

        // update bullets
        for bullet in self.bullets.iter_mut() {
            bullet.pos += bullet.vel;
        }

        self.player.update(delta);

        self.bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t);
        self.bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t && !bullet.collided);
    }

    pub fn render(&self) {
        clear_background(BLACK);
        // bullets
        for bullet in self.bullets.iter() {
            let pos1 = bullet.pos - bullet.vel;
            let pos2 = bullet.pos - bullet.vel * 2.;
            draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 1., YELLOW);
            // draw_circle(bullet.pos.x, bullet.pos.y, 1., RED)
        }
        // player
        self.player.render();
    }
}
