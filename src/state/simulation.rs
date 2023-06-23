use nalgebra as na;
use rand::Rng;

pub struct Particle {
    pub mass: f32,
    pub radius: f32,
    pub position: na::Point3<f32>,
    pub velocity: na::Vector3<f32>,
}

pub struct Simulation {
    pub particles: Vec<Particle>,
}

impl Simulation {
    pub fn new(num_particles: u32, particle_mass: f32, particle_radius: f32) -> Self {
        Self {
            particles: Self::create_particles(num_particles, particle_mass, particle_radius),
        }
    }

    fn create_particles(
        num_particles: u32,
        particle_mass: f32,
        particle_radius: f32,
    ) -> Vec<Particle> {
        let mut particles = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..num_particles {
            particles.push(Particle {
                mass: particle_mass,
                radius: particle_radius,
                position: na::Point3::new(
                    rng.gen::<f32>() * 2.0 - 1.0,
                    rng.gen::<f32>() * 2.0 - 1.0,
                    0.0,
                ),
                velocity: na::Vector3::new(
                    rng.gen::<f32>() * 2.0 - 1.0,
                    rng.gen::<f32>() * 2.0 - 1.0,
                    0.0,
                ),
            })
        }
        particles
    }

    pub fn step(&mut self) {
        for particle in &mut self.particles {
            particle.position[0] += 0.01;
            if particle.position[0] > 1.0 {
                particle.position[0] -= 2.0;
            }
            particle.position[1] += 0.01;
            if particle.position[1] > 1.0 {
                particle.position[1] -= 2.0;
            }
        }
    }
}
