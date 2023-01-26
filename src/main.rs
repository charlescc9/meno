use std::fmt;
use rand::prelude::*;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Velocity {
    speed: f64,
    direction: Point,
}

#[derive(Debug)]
struct Particle {
    id: u32,
    mass: f64,
    radius: f64,
    position: Point,
    velocity: Velocity,
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Particle {}, position ({:.2}, {:.2})", self.id, self.position.x, self.position.y)
    }
}

fn generate_particle(id: u32) -> Particle {
    let mut gen = rand::thread_rng();
    Particle {
        id,
        mass: gen.gen_range(0.0..100.0),
        radius: gen.gen_range(0.0..10.0),
        position: Point { x: gen.gen_range(0.0..100.0), y: gen.gen_range(0.0..100.0) },
        velocity: Velocity {
            speed: gen.gen_range(0.0..10.0),
            direction: Point { x: gen.gen_range(0.0..1.0), y: gen.gen_range(0.0..1.0) },
        },
    }
}

fn main() {
    let mut particles: Vec<Particle> = Vec::new();
    for i in 1..=10 {
        let particle = generate_particle(i);
        println!("Generating new particle: {}", particle);
        particles.push(particle);
    }
}
