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
            "Particle {}:
        Mass: {:.2}kg
        Radius: {:.2}m
        Position: ({:.2}m, {:.2}m)
        Velocity: {:.2}m/s, ({:.2}m, {:.2}m)",
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
    particles: Vec<Particle>,
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let positions: Vec<&Point> = self.particles.iter().map(|p| &p.position).collect();
        writeln!(f, "_________________________________________").unwrap();
        for i in 0..self.height {
            for j in 0..self.width {
                let any_local = positions.iter().any(|p| {
                    p.y > i as f64 && p.y < (i + 1) as f64 && p.x > j as f64 && p.x < (j + 1) as f64
                });
                if any_local {
                    write!(f, "| . ").unwrap();
                } else {
                    write!(f, "|   ").unwrap();
                }
            }
            write!(f, "|\n").unwrap();
        }
        write!(f, "-----------------------------------------")
    }
}

fn generate_particle(id: u32) -> Particle {
    let mut gen = rand::thread_rng();
    Particle {
        id,
        mass: gen.gen_range(0.0..100.0),
        radius: gen.gen_range(0.0..10.0),
        position: Point {
            x: gen.gen_range(0.0..10.0),
            y: gen.gen_range(0.0..10.0),
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
    }

    let space = Space {
        height: 10,
        width: 10,
        particles: particles,
    };

    println!("{}", space)
}
