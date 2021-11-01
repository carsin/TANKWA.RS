use macroquad::prelude::*;
use super::player;

pub const BULLET_WIDTH: f32 = 1.;
pub const BULLET_SPEED: f32 = 10.;

pub struct Game {
    player: player::Player,
    pub bullets: Vec<Bullet>,
    colliders: Vec<Aabb>,
}

pub struct Bullet {
    pub pos: Vec2,
    pub dir: Vec2,
    pub vel: Vec2,
    pub shot_at: f64,
    pub collider: Aabb,
    pub collided: bool,
}

pub struct Aabb {
    // 1 pos + w & h or 2 positions?
    pub pos1: Vec2,
    pub pos2: Vec2,
}

impl Game {
    pub fn new() -> Self {
        let thing1 = Aabb {
            pos1: vec2(screen_width() / 4., (screen_height() / 4.) - 10.),
            pos2: vec2(3. * screen_width() / 4., (screen_height() / 4.) + 10.),
        };

        // let thing2 = Aabb {
        //     pos1: vec2(200., 300.),
        //     pos2: vec2(800., 400.)
        // };

        let colliders = vec![thing1];
        Game {
            player: player::Player::new(),
            bullets: Vec::new(),
            colliders,
        }
    }

    pub fn update(&mut self) {
        let delta = get_frame_time();
        let frame_t = get_time();

        // shoot bullet on click
        // TODO: get position always & move pos to player obj
        if is_mouse_button_down(MouseButton::Left) && frame_t - self.player.last_shot > 0.1 {
            self.player.last_shot = frame_t;
            self.bullets.push(self.player.shoot());
        }

        // update bullets
        for bullet in self.bullets.iter_mut() {
            // move pos and colliders
            bullet.pos += bullet.vel;
            bullet.collider.pos1 += bullet.vel;
            bullet.collider.pos2 += bullet.vel;
        }

        self.player.update(delta);
        self.bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t && !bullet.collided);
    }

    pub fn render(&self) {
        clear_background(BLACK);
        // map
        for col in self.colliders.iter() {
            draw_rectangle(col.pos1.x, col.pos1.y, (col.pos2.x - col.pos1.x).abs(), (col.pos2.y - col.pos1.y).abs(), BLUE);
        }

        // bullets
        for bullet in self.bullets.iter() {
            let pos1 = bullet.pos - bullet.vel;
            let pos2 = bullet.pos - bullet.vel * 2.;
            draw_line(pos1.x, pos1.y, pos2.x, pos2.y, BULLET_WIDTH, WHITE);
            draw_line(bullet.collider.pos1.x, bullet.collider.pos1.y, bullet.collider.pos2.x, bullet.collider.pos2.y, 1., RED); // DEBUG: draw collider
        }

        // player
        self.player.render();
    }
}

impl Aabb {
    pub fn is_colliding(&self, other: &Aabb) -> boolean {
        if (self.pos1.x < self.pos2.x && self.pos2.x > 1)
    }
}
