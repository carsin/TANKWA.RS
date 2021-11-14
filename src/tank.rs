use super::game::BULLET_SPEED;
use super::projectile::Projectile;
use macroquad::prelude::*;

const PLAYER_ACCEL: f32 = 10.0;
const MAX_SPEED: f32 = 4.0;
const MAX_SPEED_VEC: Vec2 = const_vec2!([MAX_SPEED, MAX_SPEED]);
const FRICTION: Vec2 = const_vec2!([2., 1.]);

#[derive(Copy, Clone)]
pub struct Tank {
    pub pos: Vec2,
    pub size: Vec2,
    pub vel: Vec2,
    pub last_shot: f64,
    pub rot: f32,
    pub gun_dir: Vec2,
    pub gun_length: f32,
}

impl Tank {
    pub fn new(size: Vec2) -> Self {
        Tank {
            pos: vec2(screen_width() / 2., screen_height() / 2.),
            size,
            vel: vec2(0., 0.),
            last_shot: get_time(),
            rot: 0.,
            gun_dir: vec2(0., 0.),
            gun_length: size.y / 1.6,
        }
    }

    pub fn update(&mut self, delta: f32) {
        let mouse_pos = mouse_position();
        self.gun_dir = (vec2(mouse_pos.0, mouse_pos.1) - self.pos).normalize();
        // TODO: Abstract to player controller
        let rot_rads = self.rot.to_radians() + std::f32::consts::PI * 0.5;
        let dir_vec = vec2(rot_rads.cos(), rot_rads.sin());
        let dir_vec_perp = dir_vec.perp();

        // Rotational movement control
        self.rot = if is_key_down(KeyCode::A) {
            self.rot - 3.
        } else if is_key_down(KeyCode::D) {
            self.rot + 3.
        } else {
            self.rot
        };

        // Combine acceleration controls
        let forward_move = if is_key_down(KeyCode::W) { PLAYER_ACCEL } else { 0.0 };
        let backward_move = if is_key_down(KeyCode::S) { PLAYER_ACCEL } else { 0.0 };
        let accel = (forward_move - backward_move) * dir_vec;
        self.vel += accel * delta;

        // Apply friction
        let forward_mag = self.vel.dot(dir_vec);
        let sideways_mag = self.vel.dot(dir_vec_perp);
        self.vel -= dir_vec * forward_mag * delta * FRICTION.y;
        self.vel -= dir_vec_perp * sideways_mag * delta * FRICTION.x;
        self.pos += self.vel; // Make move
    }

    pub fn shoot(self) -> Projectile {
        Projectile {
            pos: self.pos + self.gun_length * self.gun_dir,
            dir: self.gun_dir,
            size: vec2(10., 20.),
            vel: BULLET_SPEED * self.gun_dir,
            shot_at: get_time(),
            collided: false,
        }
    }
}


fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
