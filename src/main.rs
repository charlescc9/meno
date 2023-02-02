use rand::prelude::*;
use std::{fmt, thread, time};

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

impl Particle {
    fn update(&mut self) -> () {
        self.position.x += self.velocity.speed * self.velocity.direction.x;
        if self.position.x < 0.0 {
            self.position.x = 10.0;
        }
        if self.position.x > 10.0 {
            self.position.x = 0.0;
        }
        self.position.y += self.velocity.speed * self.velocity.direction.y;
        if self.position.y < 0.0 {
            self.position.y = 10.0;
        }
        if self.position.y > 10.0 {
            self.position.y = 0.0;
        }
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

fn create_particle(id: u32) -> Particle {
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
            speed: gen.gen_range(0.0..1.0),
            direction: Point {
                x: gen.gen_range(-1.0..1.0),
                y: gen.gen_range(-1.0..1.0),
            },
        },
    }
}

fn main() {
    // Create particles
    let mut particles: Vec<Particle> = Vec::new();
    for i in 1..=10 {
        let particle = create_particle(i);
        particles.push(particle);
    }

    // Initialize space
    let mut space = Space {
        height: 10,
        width: 10,
        particles: particles,
    };
    println!("{}", space);

    // Move particles
    loop {
        for particle in &mut space.particles {
            particle.update();
        }
        println!("{}", space);
        thread::sleep(time::Duration::from_millis(100));
    }
}
