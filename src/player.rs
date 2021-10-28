use macroquad::prelude::*;

#[derive(Copy, Clone)]
pub struct Player {
    pub pos: Vec2,
    pub size: Vec2,
    pub rot: f32,
    pub vel: Vec2,
}

impl Player {
    pub fn new() -> Self {
        Player {
            pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
            rot: 0.,
            vel: Vec2::new(0., 0.),
            size: Vec2::new(20., 20.),
        }
    }

    pub fn update(&mut self) {
        let rotation = self.rot.to_radians();
        let mut acc = self.vel / 10.;

        self.vel += acc;
        if self.vel.length() > 5. {
            self.vel = self.vel.normalize() * 5.;
        }
        self.pos += self.vel;
    }

    pub fn render(self) {
        draw_rectangle(self.pos.x - self.size.x / 2., self.pos.y - self.size.y / 2., self.size.x, self.size.y, BLACK);
        draw_rectangle(self.pos.x - 1., self.pos.y - 1., 2., 2., RED);
    }
}
