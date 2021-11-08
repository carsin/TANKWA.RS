use macroquad::prelude::*;
use super::tank::Tank;

pub const BULLET_WIDTH: f32 = 1.;
pub const BULLET_SPEED: f32 = 10.;

pub struct Game {
    player: Tank,
    pub projectiles: Vec<Projectile>,
    // TODO: Change to map
    textures: Vec<Texture2D>,
}

pub struct Projectile {
    pub pos: Vec2,
    pub dir: Vec2,
    pub vel: Vec2,
    pub size: Vec2,
    pub shot_at: f64,
    pub collided: bool,
}

impl Game {
    pub fn new(textures: Vec<Texture2D>) -> Self {
        Game {
            player: Tank::new(),
            projectiles: Vec::new(),
            textures,
        }
    }

    pub fn update(&mut self) {
        let delta = get_frame_time();
        let frame_t = get_time();

        // shoot bullet on click
        if is_mouse_button_down(MouseButton::Left) && frame_t - self.player.last_shot > 0.1 {
            self.player.last_shot = frame_t;
            self.projectiles.push(self.player.shoot());
        }

        // update bullets
        for proj in self.projectiles.iter_mut() {
            // move pos and colliders
            proj.pos += proj.vel;
        }

        self.player.update(delta);
        self.projectiles.retain(|bullet| bullet.shot_at + 1.5 > frame_t && !bullet.collided);
    }

    pub fn render(&self) {
        clear_background(BLACK);

        // projectiles
        for proj in self.projectiles.iter() {
            let pos1 = proj.pos - proj.vel;
            let pos2 = proj.pos - proj.vel * 2.;
            draw_line(pos1.x, pos1.y, pos2.x, pos2.y, BULLET_WIDTH, YELLOW);
        }

        // player
        // tank
        // let rot_angle = self.player.dir.y.atan2(self.player.dir.x);
        draw_texture_ex(self.textures[0], self.player.pos.x - self.player.size.x / 2., self.player.pos.y - self.player.size.y / 2., WHITE, DrawTextureParams {
            dest_size: Some(self.player.size),
            source: None,
            // rotation: 0.,
            rotation: self.player.rot.to_radians(),
            flip_x: false,
            flip_y: false,
            pivot: None,
        });

        // gun barrel
        draw_line(self.player.pos.x, self.player.pos.y, self.player.pos.x + self.player.gun_dir.x * self.player.gun_length, self.player.pos.y + self.player.gun_dir.y * self.player.gun_length, 10., GRAY);

        // center turret
        draw_circle(self.player.pos.x, self.player.pos.y, self.player.size.x / 2.5, GRAY);


        // self.player.render();
    }
}
