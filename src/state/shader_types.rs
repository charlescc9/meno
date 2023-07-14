use super::simulation;
use std::f32::consts::TAU;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ParticleRaw {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
}

impl ParticleRaw {
    pub fn convert(particles: &Vec<simulation::Particle>, particles_raw: &mut Vec<ParticleRaw>) {
        let paricles_positions: Vec<[f32; 3]> = particles
            .iter()
            .map(|p| p.position.as_slice().try_into().unwrap())
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

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexRaw {
    pub offset: [f32; 3],
    pub color: [f32; 3],
}

impl VertexRaw {
    pub fn create_particles_vertices(num_sides: u32, radius: f32) -> (Vec<VertexRaw>, Vec<u32>) {
        let mut vertices = vec![VertexRaw {
            offset: [0.0, 0.0, 0.0],
            color: [1.0, 0.0, 0.0],
        }];
        let mut indices = Vec::new();

        for i in 1..num_sides + 1 {
            let x = radius * f32::cos(i as f32 * TAU / num_sides as f32);
            let y = radius * f32::sin(i as f32 * TAU / num_sides as f32);
            vertices.push(VertexRaw {
                offset: [x, y, 0.0],
                color: [1.0, 0.0, 0.0],
            });
            indices.extend([0, i, (i % num_sides) + 1]);
        }

        (vertices, indices)
    }
}
