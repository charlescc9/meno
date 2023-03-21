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
        writeln!(f, "{}", "-".repeat(self.width as usize * 4 + 1)).unwrap();

        let positions: Vec<&Point> = self.particles.iter().map(|p| &p.position).collect();
        for i in 0..self.height {
            for j in 0..self.width {
                let point_at_location = positions.iter().enumerate().find(|(_, p)| 
                p.y > i as f64 && p.y < (i + 1) as f64 && p.x > j as f64 && p.x < (j + 1) as f64 );
                if let Some((i, _)) = point_at_location {
                    write!(f, "| {} ", i).unwrap();
                } else {
                    write!(f, "|   ").unwrap();
                }
            }
            write!(f, "|\n").unwrap();
        }
        
        writeln!(f, "{}", "-".repeat(self.width as usize * 4 + 1))
    }
}
