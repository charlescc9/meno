use rand::prelude::*;
use std::fmt;

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
        write!(
            f,
            "Particle {}:\n\tMass: {:.2}kg\n\tRadius: {:.2}m\n\tPosition: ({:.2}m, {:.2}m)\n\tVelocity: {:.2}m/s, ({:.2}m, {:.2}m)",
            self.id,
            self.mass,
            self.radius,
            self.position.x,
            self.position.y,
            self.velocity.speed,
            self.velocity.direction.x,
            self.velocity.direction.y
        )
    }
}

struct Space {
    height: u32,
    width: u32,
    particles: Vec<Particle>
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "test")
    }
}

fn generate_particle(id: u32) -> Particle {
    let mut gen = rand::thread_rng();
    Particle {
        id,
        mass: gen.gen_range(0.0..100.0),
        radius: gen.gen_range(0.0..10.0),
        position: Point {
            x: gen.gen_range(0.0..100.0),
            y: gen.gen_range(0.0..100.0),
        },
        velocity: Velocity {
            speed: gen.gen_range(0.0..10.0),
            direction: Point {
                x: gen.gen_range(0.0..1.0),
                y: gen.gen_range(0.0..1.0),
            },
        },
    }
}

fn main() {
    let mut particles: Vec<Particle> = Vec::new();
    for i in 1..=10 {
        let particle = generate_particle(i);
        println!("Generating new particle:\n  {}", particle);
        particles.push(particle);
    };

    let space = Space {
        height: 10,
        width: 10,
        particles: particles
    };

    println!("Space: {}", space)
}
