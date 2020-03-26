use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use rand::Rng;

mod physics;

use physics::*;

pub struct App {
    particles: Vec<Particle>,
}

impl App {
    fn new() -> App {
        let mut app = App {
            particles: vec![Particle::new(0.0, 0.0); NUM_PARTICLES],
        };
        let mut rng = rand::thread_rng();
        for i in 0..NUM_PARTICLES {
            app.particles[i].pos = (rng.gen::<f64>() * 800.0, rng.gen::<f64>() * 800.0);
            // app.particles[i].vel = (rng.gen::<f64>() - 0.5, rng.gen::<f64>() - 0.5);
        }
        app
    }

    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;

        gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear([0.0, 0.0, 0.0, 1.0], gl);
            for particle in self.particles.iter() {
                particle.draw(&c, gl);
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        'outer: for j in 0..self.particles.len() {
            self.particles[j].accel = (0.0, 0.0);
            for i in 0..self.particles.len() {
                if j == i {
                    continue;
                }
                let a = self.particles[i].center.1 - self.particles[j].center.1;
                let b = self.particles[i].center.0 - self.particles[j].center.0;
                let radius_squared = (a).powi(2) + (b).powi(2);
                let force =
                    (GRAVITATIONAL_CONSTANT * self.particles[j].mass * self.particles[i].mass)
                        / radius_squared;
                let angle = a.atan2(b);
                if radius_squared
                    >= ((self.particles[i].mass + self.particles[j].mass) / 2.0).powi(2)
                {
                    self.particles[j].accel = (
                        self.particles[j].accel.0
                            + (angle.cos() * (force / self.particles[j].mass)),
                        self.particles[j].accel.1
                            + (angle.sin() * (force / self.particles[j].mass)),
                    );
                } else if COLLISIONS {
                    let total_mass = self.particles[i].mass + self.particles[j].mass;
                    let total_momentum = (
                        self.particles[i].vel.0 * self.particles[i].mass
                            + self.particles[j].vel.0 * self.particles[j].mass,
                        self.particles[i].vel.1 * self.particles[i].mass
                            + self.particles[j].vel.1 * self.particles[j].mass,
                    );
                    self.particles[j].vel =
                        (total_momentum.0 / total_mass, total_momentum.1 / total_mass);
                    if self.particles[i].mass > self.particles[j].mass {
                        self.particles[j].pos = self.particles[i].pos;
                    }
                    self.particles[j].mass = total_mass;
                    self.particles.remove(i);
                    break 'outer;
                }
            }
        }

        for p in self.particles.iter_mut() {
            p.update();
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V4_5;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Hydrogen Ferride", [800, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new();

    let mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &mut gl);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
