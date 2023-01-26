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
        write!(f, "Particle {} at position ({}, {})", self.id, self.position.x, self.position.y)
    }
}

fn main() {
    let particle = Particle {
        id: 1,
        mass: 1.0,
        radius: 1.0,
        position: Point { x: 1.1, y: 2.2 },
        velocity: Velocity {
            speed: 1.0,
            direction: Point { x: 1.0, y: 1.0 },
        },
    };

    println!("{}", particle);
}
