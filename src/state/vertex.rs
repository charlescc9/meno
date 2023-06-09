use std::f32::consts::TAU;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub offset: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn create_particles(particle_radius: f32, particle_sides: u32) -> (Vec<Vertex>, Vec<u32>) {
        let mut vertices = vec![Vertex {
            offset: [0.0, 0.0, 0.0],
            color: [1.0, 0.0, 0.0],
        }];
        let mut indices = Vec::new();

        for i in 1..particle_sides + 1 {
            let x = particle_radius * f32::cos(i as f32 * TAU / particle_sides as f32);
            let y = particle_radius * f32::sin(i as f32 * TAU / particle_sides as f32);
            vertices.push(Vertex {
                offset: [x, y, 0.0],
                color: [1.0, 0.0, 0.0],
            });
            indices.extend([0, i, (i % particle_sides) + 1]);
        }

        (vertices, indices)
    }
}
