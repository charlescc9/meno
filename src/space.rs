use std::fmt;

use crate::particle::Particle;
use crate::point::Point;

#[derive(Debug)]
pub struct Space {
    pub height: u32,
    pub width: u32,
    pub particles: Vec<Particle>,
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let positions: Vec<&Point> = self.particles.iter().map(|p| &p.position).collect();
        writeln!(f, "{}", "_".repeat(self.width as usize * 4 + 1)).unwrap();
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
        writeln!(f, "{}", "-".repeat(self.width as usize * 4 + 1))
    }
}
