use macroquad::prelude::*;
use super::projectile::Projectile;
use super::game::BULLET_SPEED;

const PLAYER_ACCEL: f32 = 15.0;
const MAX_SPEED: f32 = 7.0;
const MAX_SPEED_VEC: Vec2 = const_vec2!([MAX_SPEED, MAX_SPEED]);
const FRICTION: Vec2 = const_vec2!([0.3, 0.9]);

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
        let rot_rads = self.rot.to_radians();
        let rot_vec = vec2(rot_rads.sin(), -rot_rads.cos());
        self.vel = if is_key_down(KeyCode::W) {
            (self.vel + rot_vec * PLAYER_ACCEL * delta).min(MAX_SPEED_VEC).max(-MAX_SPEED_VEC)
        } else if is_key_down(KeyCode::S) {
            (self.vel - rot_vec * PLAYER_ACCEL * delta).max(-MAX_SPEED_VEC).min(MAX_SPEED_VEC)
        } else {
            // TODO: Fix friction values per side of tank
            self.vel * FRICTION
        };

        // Rotational movement control
        self.rot = if is_key_down(KeyCode::Q) {
            self.rot - 3.
        } else if is_key_down(KeyCode::E) {
            self.rot + 3.
        } else {
            self.rot
        };

        self.pos += self.vel;
    }

    pub fn shoot(self) -> Projectile {
        Projectile {
            pos: self.pos + self.gun_length * self.gun_dir,
            dir: self.gun_dir,
            size: vec2(10., 20.,),
            vel: BULLET_SPEED * self.gun_dir,
            shot_at: get_time(),
            collided: false,
        }
    }
}
