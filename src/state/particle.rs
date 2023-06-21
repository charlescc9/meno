use nalgebra as na;
use rand::Rng;

pub struct Particle {
    pub position: na::Point3<f32>,
    pub velocity: na::Vector3<f32>,
}

impl Particle {
    pub fn create_particles(num_particles: u32) -> Vec<Particle> {
        let mut particles = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..num_particles {
            particles.push(Particle {
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
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ParticleRaw {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
}

impl ParticleRaw {
    pub fn convert(particles: &Vec<Particle>, particles_raw: &mut Vec<ParticleRaw>) {
        let paricles_positions: Vec<[f32; 3]> = particles
            .iter()
            .map(|p| p.position.coords.as_slice().try_into().unwrap())
            .collect();
        let paricles_velocities: Vec<[f32; 3]> = particles
            .iter()
            .map(|p| p.velocity.as_slice().try_into().unwrap())
            .collect();
        for i in 0..particles.len() {
            if particles_raw.len() > i {
                particles_raw[i].position = paricles_positions[i];
                particles_raw[i].velocity = paricles_velocities[i];
            } else {
                particles_raw.push(ParticleRaw {
                    position: paricles_positions[i],
                    velocity: paricles_velocities[i],
                })
            }
        }
    }
}
