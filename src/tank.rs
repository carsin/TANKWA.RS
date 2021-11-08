use macroquad::prelude::*;
use crate::game::{Projectile, BULLET_SPEED};

const PLAYER_ACCEL: f32 = 27.0;
const FRICTION: f32 = 0.77;
const MAX_SPEED: f32 = 9.0;

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
    pub fn new() -> Self {
        Tank {
            pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
            size: Vec2::new(30., 45.),
            vel: Vec2::new(0., 0.),
            last_shot: get_time(),
            rot: 0.,
            gun_dir: Vec2::new(0., 0.),
            gun_length: 30.,
        }
    }

    pub fn update(&mut self, delta: f32) {
        let mouse_pos = mouse_position();
        self.gun_dir = (vec2(mouse_pos.0, mouse_pos.1) - self.pos).normalize();
        // TODO: Abstract to player controller
        // TODO: compute velocity from rotation
        // Vertical movement control
        self.vel.y = if is_key_down(KeyCode::W) {
            (self.vel.y - PLAYER_ACCEL * delta).max(-MAX_SPEED)
        } else if is_key_down(KeyCode::S) {
            (self.vel.y + PLAYER_ACCEL * delta).min(MAX_SPEED)
        } else {
            self.vel.y * FRICTION
        };

        // Horizontal movement control
        self.vel.x = if is_key_down(KeyCode::A) {
            (self.vel.x - PLAYER_ACCEL * delta).max(-MAX_SPEED)
        } else if is_key_down(KeyCode::D) {
            (self.vel.x + PLAYER_ACCEL * delta).min(MAX_SPEED)
        } else {
            self.vel.x * FRICTION
        };

        // Rotational movement control
        self.rot = if is_key_down(KeyCode::Q) {
            self.rot - 5.
        } else if is_key_down(KeyCode::E) {
            self.rot + 5.
        } else {
            self.rot
        };

        self.pos += self.vel;
    }

    pub fn render(self) {
        // draw_rectangle(self.pos.x - self.size.x / 2., self.pos.y - self.size.y / 2., self.size.x, self.size.y, WHITE);
        draw_poly(self.pos.x, self.pos.y, 4, self.size.x, 0.0, WHITE);
        draw_line(self.pos.x, self.pos.y, self.pos.x + self.gun_dir.x * self.gun_length, self.pos.y + self.gun_dir.y * self.gun_length, 10., BLUE);
        draw_circle(self.pos.x, self.pos.y, 1.0, RED);
    }

    pub fn shoot(self) -> Projectile {
        let position = self.pos + self.gun_dir * self.size.x * 1.5;
        Projectile {
            pos: position + self.gun_dir * self.gun_length,
            dir: self.gun_dir,
            size: Vec2::new(10., 20.,),
            vel: BULLET_SPEED * self.gun_dir,
            shot_at: get_time(),
            // collider is at tip of bullet, BULLET_WIDTH wide
            collided: false,
        }
    }
}
