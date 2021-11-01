use macroquad::prelude::*;

const PLAYER_ACCEL: f32 = 23.0;
const FRICTION: f32 = 0.77;
const MAX_SPEED: f32 = 9.0;

#[derive(Copy, Clone)]
pub struct Player {
    pub pos: Vec2,
    pub size: Vec2,
    pub rot: f32,
    pub vel: Vec2,
    pub last_shot: f64,
}

impl Player {
    pub fn new() -> Self {
        Player {
            pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
            rot: 0.,
            vel: Vec2::new(0., 0.),
            size: Vec2::new(20., 20.),
            last_shot: get_time(),
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.vel.y = if is_key_down(KeyCode::W) {
            (self.vel.y - PLAYER_ACCEL * delta).max(-MAX_SPEED)
        } else if is_key_down(KeyCode::S) {
            (self.vel.y + PLAYER_ACCEL * delta).min(MAX_SPEED)
        } else {
            self.vel.y * FRICTION
        };

        self.vel.x = if is_key_down(KeyCode::A) {
            (self.vel.x - PLAYER_ACCEL * delta).max(-MAX_SPEED)
        } else if is_key_down(KeyCode::D) {
            (self.vel.x + PLAYER_ACCEL * delta).min(MAX_SPEED)
        } else {
            self.vel.x * FRICTION
        };

        self.pos += self.vel;
    }

    pub fn render(self) {
        draw_rectangle(self.pos.x - self.size.x / 2., self.pos.y - self.size.y / 2., self.size.x, self.size.y, WHITE);
        draw_circle(self.pos.x, self.pos.y, 1.0, RED);
    }
}
