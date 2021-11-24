use macroquad::prelude::*;

pub struct Projectile {
    pub pos: Vec2,
    pub dir: Vec2,
    pub vel: Vec2,
    pub size: Vec2, // TODO: make this work
    pub shot_at: f64,
    pub collided: bool,
}

impl Projectile {
    pub fn update(&mut self) {
        self.pos += self.vel; // move
    }
}
