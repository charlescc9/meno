use std::fmt;

use crate::components::Components;
use crate::particle::Particle;

#[derive(Debug)]
pub struct Space {
    pub height: u32,
    pub width: u32,
    pub particles: Vec<Particle>,
    pub time: u32,
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Space at time {}:", self.time)?;
        writeln!(f, "{}", "-".repeat(self.width as usize * 4 + 1))?;

        let positions: Vec<&Components> = self.particles.iter().map(|p| &p.position).collect();
        for i in 0..self.height {
            for j in 0..self.width {
                let point_at_location = positions.iter().enumerate().find(|(_, p)| {
                    p.y > i as f64 && p.y < (i + 1) as f64 && p.x > j as f64 && p.x < (j + 1) as f64
                });

                if let Some((i, _)) = point_at_location {
                    write!(f, "| {} ", i)?;
                } else {
                    write!(f, "|   ")?;
                }
            }
            write!(f, "|\n")?;
        }

        writeln!(f, "{}", "-".repeat(self.width as usize * 4 + 1))
    }
}
