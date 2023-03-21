use rand::prelude::*;
use std::fmt;

use crate::math::get_random_direction;
use crate::point::Point;
use crate::vector::Vector;

#[derive(Debug)]
pub struct Particle {
    pub id: u32,
    pub mass: f64,
    pub position: Point,
    pub velocity: Vector,
    pub gravitational_potential: f64,
}

impl Particle {
    pub fn new(
        gen: &mut ThreadRng,
        id: u32,
        width: u32,
        height: u32,
        max_mass: f64,
        max_speed: f64,
    ) -> Self {
        Particle {
            id,
            mass: gen.gen_range(0.0..max_mass),
            position: Point {
                x: gen.gen_range(0.0..width as f64),
                y: gen.gen_range(0.0..height as f64),
            },
            velocity: Vector {
                magnitude: gen.gen_range(0.0..max_speed),
                direction: get_random_direction(gen),
            },
            gravitational_potential: 0.0,
        }
    }

    pub fn update(&mut self, width: u32, height: u32, gravitational_potential: f64) -> () {
        self.position.x += self.velocity.magnitude * self.velocity.direction.x;
        if self.position.x < 0.0 {
            self.position.x = width as f64;
        }
        if self.position.x > width as f64 {
            self.position.x = 0.0;
        }

        self.position.y += self.velocity.magnitude * self.velocity.direction.y;
        if self.position.y < 0.0 {
            self.position.y = height as f64;
        }
        if self.position.y > height as f64 {
            self.position.y = 0.0;
        }

        self.gravitational_potential = gravitational_potential;
    }
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Particle {}:
        Mass: {:.2}kg
        Position: ({:.2}m, {:.2}m)
        Velocity: {:.2}m/s, ({:.2}m, {:.2}m)",
            self.id,
            self.mass,
            self.position.x,
            self.position.y,
            self.velocity.magnitude,
            self.velocity.direction.x,
            self.velocity.direction.y
        )
    }
}
