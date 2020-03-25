use graphics::*;
use opengl_graphics::GlGraphics;

pub const PARTICLE_SIZE: f64 = 1.0;
pub const NUM_PARTICLES: usize = 200;
pub const COLLISIONS: bool = true;
pub const GRAVITATIONAL_CONSTANT: f64 = 3.0;

#[derive(Clone, Copy)]
pub struct Particle {
    pub pos: (f64, f64),
    pub center: (f64, f64),
    pub accel: (f64, f64),
    pub vel: (f64, f64),
    pub mass: f64,
}

impl Particle {
    pub fn new(x: f64, y: f64) -> Particle {
        Particle {
            pos: (x, y),
            center: (x + (PARTICLE_SIZE / 2.0), y + (PARTICLE_SIZE / 2.0)),
            accel: (0.0, 0.0),
            vel: (0.0, 0.0),
            mass: 1.0,
        }
    }

    pub fn draw(&self, c: &Context, g: &mut GlGraphics) {
        let transform = c.transform;
        ellipse(
            [0.0, 1.0, 0.0, 1.0],
            rectangle::square(self.pos.0, self.pos.1, self.mass),
            transform,
            g,
        );
    }

    pub fn update(&mut self) {
        self.vel.0 += self.accel.0;
        self.vel.1 += self.accel.1;
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
        self.center.0 = self.pos.0 + (self.mass / 2.0);
        self.center.1 = self.pos.1 + (self.mass / 2.0);
    }
}
