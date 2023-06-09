use rand::Rng;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Particle {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
}

impl Particle {
    pub fn create_particles(num_particles: u32) -> Vec<Particle> {
        let mut particles = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..num_particles {
            particles.push(Particle {
                position: [
                    rng.gen::<f32>() * 2.0 - 1.0,
                    rng.gen::<f32>() * 2.0 - 1.0,
                    0.0,
                ],
                velocity: [
                    (rng.gen::<f32>() * 2.0 - 1.0) * 0.1,
                    (rng.gen::<f32>() * 2.0 - 1.0) * 0.1,
                    0.0,
                ],
            })
        }
        particles
    }
}
