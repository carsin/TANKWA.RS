use macroquad::prelude::*;

pub struct Projectile {
    pub pos: Vec2,
    pub dir: Vec2,
    pub vel: Vec2,
    pub size: Vec2,
    pub shot_at: f64,
    pub collided: bool,
}
