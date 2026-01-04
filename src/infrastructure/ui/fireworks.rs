use macroquad::color::Color;
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{draw_circle, rand, screen_height, screen_width};

pub struct Particle {
    pos: Vec2,
    vel: Vec2,
    color: Color,
    life: f32,
}

impl Particle {
    pub fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt;
        self.vel *= 0.98; // friction légère
        self.life -= dt;
    }

    pub fn draw(&self) {
        let alpha = self.life.clamp(0.0, 1.0);
        let mut c = self.color;
        c.a = alpha;
        draw_circle(self.pos.x, self.pos.y, 3.0, c);
    }
    
    pub fn life(&self) -> f32 {
        self.life
    }
}

pub fn spawn_firework(particles: &mut Vec<Particle>) {
    let center = vec2(
        rand::gen_range(100.0, screen_width() - 100.0),
        rand::gen_range(100.0, screen_height() - 200.0),
    );

    let base_color = Color::new(
        rand::gen_range(0.5, 1.0),
        rand::gen_range(0.5, 1.0),
        rand::gen_range(0.5, 1.0),
        1.0,
    );

    for _ in 0..40 {
        let angle = rand::gen_range(0.0, std::f32::consts::TAU);
        let speed = rand::gen_range(80.0, 200.0);

        particles.push(Particle {
            pos: center,
            vel: vec2(angle.cos() * speed, angle.sin() * speed),
            color: base_color,
            life: rand::gen_range(0.8, 1.5),
        });
    }
}