use rand::prelude::*;
use std::fmt;

use crate::components::Components;

#[derive(Debug)]
pub struct Particle {
    pub id: u32,
    pub mass: f64,
    pub position: Components,
    pub velocity: Components,
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
            position: Components {
                x: gen.gen_range(0.0..width as f64),
                y: gen.gen_range(0.0..height as f64),
            },
            velocity: Components {
                x: gen.gen_range(0.0..max_speed),
                y: gen.gen_range(0.0..max_speed),
            },
        }
    }

    pub fn update(&mut self, width: u32, height: u32, gravitational_force: Components) -> () {
        let acceleration_x = gravitational_force.x / self.mass;
        let acceleration_y = gravitational_force.y / self.mass;

        self.velocity.x += acceleration_x / 2.0;
        self.velocity.y += acceleration_y / 2.0;

        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        
        // Check bounds
        if self.position.x < 0.0 {
            self.position.x = width as f64;
        }
        if self.position.x > width as f64 {
            self.position.x = 0.0;
        }
        if self.position.y < 0.0 {
            self.position.y = height as f64;
        }
        if self.position.y > height as f64 {
            self.position.y = 0.0;
        }
    }
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Particle {}:
        Mass: {:.2}kg
        Position: ({:.2}m, {:.2}m)
        Velocity: ({:.2}m/s, {:.2}m/s)",
            self.id,
            self.mass,
            self.position.x,
            self.position.y,
            self.velocity.x,
            self.velocity.y
        )
    }
}
