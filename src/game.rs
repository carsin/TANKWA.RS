use macroquad::prelude::*;
use super::player;

const BULLET_WIDTH: f32 = 1.;
const BULLET_SPEED: f32 = 15.;

pub struct Game {
    player: player::Player,
    pub bullets: Vec<Bullet>,
}

pub struct Bullet {
    pub pos: Vec2,
    pub dir: Vec2,
    vel: Vec2,
    shot_at: f64,
    collider: Aabb,
    collided: bool,
}

struct Aabb {
    pub pos1: Vec2,
    pub pos2: Vec2,
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
            let position = (self.player.pos) + direction;
            // TODO: move bullet spawning elsewhere
            let new_bullet = Bullet {
                pos: position + direction * self.player.size.x * 1.5,
                dir: direction,
                vel: BULLET_SPEED * direction,
                shot_at: frame_t,
                // collider is at tip of bullet, BULLET_WIDTH wide
                collider: Aabb { pos1: position - direction.perp() * (BULLET_WIDTH / 2.), pos2: position + direction.perp() * (BULLET_WIDTH / 2.) },
                collided: false,
            };
            self.bullets.push(new_bullet);
            self.player.last_shot = frame_t;
        }

        // update bullets
        for bullet in self.bullets.iter_mut() {
            bullet.pos += bullet.vel;
            bullet.collider.pos1 += bullet.vel;
            bullet.collider.pos2 += bullet.vel;
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
            draw_line(pos1.x, pos1.y, pos2.x, pos2.y, BULLET_WIDTH, YELLOW);
            draw_line(bullet.collider.pos1.x, bullet.collider.pos1.y, bullet.collider.pos2.x, bullet.collider.pos2.y, 1., RED); // DEBUG: draw collider
            // draw_circle(bullet.pos.x, bullet.pos.y, 1., RED)
        }
        // player
        self.player.render();
    }
}
