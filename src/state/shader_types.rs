use super::simulation;
use std::f32::consts::TAU;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ParticleRaw {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl ParticleRaw {
    pub fn generate_shader_particles(
        max_velocity: f32,
        particles: &Vec<simulation::Particle>,
        particles_raw: &mut Vec<ParticleRaw>,
    ) {
        let paricles_positions: Vec<[f32; 3]> = particles
            .iter()
            .map(|p| p.position.as_slice().try_into().unwrap())
            .collect();
        let particles_colors: Vec<[f32; 3]> = particles
            .iter()
            .map(|p| {
                [
                    1.0 - p.velocity.magnitude() / max_velocity,
                    0.0,
                    f32::min(p.velocity.magnitude() / max_velocity, 1.0),
                ]
            })
            .collect();
        for i in 0..particles.len() {
            if particles_raw.len() > i {
                particles_raw[i].position = paricles_positions[i];
                particles_raw[i].color = particles_colors[i];
            } else {
                particles_raw.push(ParticleRaw {
                    position: paricles_positions[i],
                    color: particles_colors[i],
                })
            }
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexRaw {
    pub offset: [f32; 3],
}

impl VertexRaw {
    pub fn generate_shader_vertices(num_sides: u32, radius: f32) -> (Vec<VertexRaw>, Vec<u32>) {
        let mut vertices = vec![VertexRaw {
            offset: [0.0, 0.0, 0.0],
        }];
        let mut indices = Vec::new();

        for i in 1..num_sides + 1 {
            let x = radius * f32::cos(i as f32 * TAU / num_sides as f32);
            let y = radius * f32::sin(i as f32 * TAU / num_sides as f32);
            vertices.push(VertexRaw {
                offset: [x, y, 0.0],
            });
            indices.extend([0, i, (i % num_sides) + 1]);
        }

        (vertices, indices)
    }
}
