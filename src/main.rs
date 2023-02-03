use rand::prelude::*;
use std::{
    f64::{self, consts::PI},
    fmt, thread, time,
};

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
    fn new(
        gen: &mut ThreadRng,
        id: u32,
        width: u32,
        height: u32,
        max_mass: f64,
        max_radius: f64,
        max_speed: f64,
    ) -> Self {
        Particle {
            id,
            mass: gen.gen_range(0.0..max_mass),
            radius: gen.gen_range(0.0..max_radius),
            position: Point {
                x: gen.gen_range(0.0..width as f64),
                y: gen.gen_range(0.0..height as f64),
            },
            velocity: Velocity {
                speed: gen.gen_range(0.0..max_speed),
                direction: Particle::get_random_direction(gen),
            },
        }
    }

    fn get_random_direction(gen: &mut ThreadRng) -> Point {
        let theta = 2.0 * PI * gen.gen_range(0.0..1.0);
        Point {
            x: theta.sin(),
            y: theta.cos(),
        }
    }

    fn update_position(&mut self, width: u32, height: u32) -> () {
        self.position.x += self.velocity.speed * self.velocity.direction.x;
        if self.position.x < 0.0 {
            self.position.x = width as f64;
        }
        if self.position.x > width as f64 {
            self.position.x = 0.0;
        }
        self.position.y += self.velocity.speed * self.velocity.direction.y;
        if self.position.y < 0.0 {
            self.position.y = height as f64;
        }
        if self.position.y > height as f64 {
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

fn main() {
    // Global variables
    let mut gen = rand::thread_rng();
    let width = 10;
    let height = 10;
    let max_mass = 10.0;
    let max_radius = 10.0;
    let max_speed = 1.0;
    let num_partiles = 10;

    // Create particles
    let mut particles: Vec<Particle> = Vec::new();
    for i in 0..num_partiles {
        let particle = Particle::new(&mut gen, i, width, height, max_mass, max_radius, max_speed);
        particles.push(particle);
    }

    // Initialize space
    let mut space = Space {
        height: height,
        width: width,
        particles: particles,
    };
    println!("{}", space);

    // Move particles
    loop {
        for particle in &mut space.particles {
            particle.update_position(width, height);
        }
        println!("{}", space);
        thread::sleep(time::Duration::from_millis(100));
    }
}
