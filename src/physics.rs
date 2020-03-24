use graphics::*;
use opengl_graphics::GlGraphics;

pub const PARTICLE_SIZE: f64 = 1.0;
pub const NUM_PARTICLES: usize = 500;

#[derive(Clone, Copy)]
pub struct Particle {
    pub pos: (f64, f64),
    pub center: (f64, f64),
    pub accel: (f64, f64),
    pub vel: (f64, f64),
}

impl Particle {
    pub fn new(x: f64, y: f64) -> Particle {
        Particle {
            pos: (x, y),
            center: (x + (PARTICLE_SIZE / 2.0), y + (PARTICLE_SIZE / 2.0)),
            accel: (0.0, 0.0),
            vel: (0.0, 0.0),
        }
    }

    pub fn draw(&self, c: &Context, g: &mut GlGraphics) {
        let transform = c.transform;
        rectangle(
            [0.0, 1.0, 0.0, 1.0],
            rectangle::square(self.pos.0, self.pos.1, PARTICLE_SIZE),
            transform,
            g,
        );
    }

    pub fn update(&mut self) {
        self.vel.0 += self.accel.0;
        self.vel.1 += self.accel.1;
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
    }
}
