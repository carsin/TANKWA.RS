use macroquad::prelude::*;

const PLAYER_ACCEL: f32 = 1.0;

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
        println!("{}", delta);
        let direction = vec2(self.rot.cos(), self.rot.sin());
        let target_vel = if is_key_down(KeyCode::W) { direction } else if is_key_down(KeyCode::S) { -direction } else { vec2(0.0, 0.0) };

        self.vel = self.vel.lerp(target_vel, delta);

        self.pos += self.vel * delta;
        // TODO: Set up if else chain of target velocities
        // at end, do target_vel * 100

        // let target_vel;
        // target_vel = if is_key_down(KeyCode::W) {
        //     self.vel.y *= PLAYER_ACCEL
        // }
        // let target_vel = if is_key_down(KeyCode::S) {
        //     self.vel.y += PLAYER_ACCEL;
        // }
        //
        // if is_key_down(KeyCode::A) {
        //     self.vel.x -= PLAYER_ACCEL;
        // } else if is_key_down(KeyCode::D) {
        //     self.vel.x += PLAYER_ACCEL;
        // }
        //
        // let target_vel = if forward {
        //     // direction * speed
        // } else {
        //     vec2(0.0, 0.0)
        // }
        //
        // self.vel = self.vel.lerp(target_vel, frame_t as f32);
        // // let rotation = self.rot.to_radians();
        // // if self.vel.length() > 5. {
        // //     self.vel = self.vel.normalize() * 5.;
        // // }
        // self.pos += self.vel;
        // // self.vel = self.vel.lerp(vec2(0., 0.), frame_t as f32);
    }

    pub fn render(self) {
        draw_rectangle(self.pos.x - self.size.x / 2., self.pos.y - self.size.y / 2., self.size.x, self.size.y, WHITE);
        draw_circle(self.pos.x, self.pos.y, 1.0, RED);
    }
}
