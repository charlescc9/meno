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
    pub fn new(num_particles: u32, max_mass: f32, max_velocity: f32, radius: f32) -> Self {
        Self {
            particles: Self::create_particles(num_particles, max_mass, max_velocity, radius),
        }
    }

    fn create_particles(
        num_particles: u32,
        max_mass: f32,
        max_velocity: f32,
        radius: f32,
    ) -> Vec<Particle> {
        let mut particles = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..num_particles {
            loop {
                let lower_limit = -1.0 + radius;
                let upper_limit = 1.0 - radius;
                let new_particle = Particle {
                    mass: rng.gen_range(0.0..=max_mass),
                    radius,
                    position: na::Point3::new(
                        rng.gen_range(lower_limit..=upper_limit),
                        rng.gen_range(lower_limit..=upper_limit),
                        0.0,
                    ),
                    velocity: na::Vector3::new(
                        rng.gen_range(0.0..=max_velocity),
                        rng.gen_range(0.0..=max_velocity),
                        0.0,
                    ),
                };
                let mut in_collision = false;
                for particle in &particles {
                    if Self::detect_overlap(&new_particle, &particle) {
                        in_collision = true;
                    }
                }
                if !in_collision {
                    particles.push(new_particle);
                    break;
                }
            }
        }
        particles
    }

    pub fn step(&mut self) {
        for particle in &mut self.particles {
            particle.position[0] += particle.velocity[0];
            particle.position[1] += particle.velocity[1];
            self::Simulation::detect_wall_collision(particle);
        }
        self.detect_particle_collisions();
    }

    fn detect_wall_collision(particle: &mut Particle) {
        if particle.position.x - particle.radius < -1.0
            || particle.position.x + particle.radius > 1.0 as f32
        {
            particle.velocity[0] *= -1.0;
        }
        if particle.position.y - particle.radius < -1.0
            || particle.position.y + particle.radius > 1.0 as f32
        {
            particle.velocity[1] *= -1.0;
        }
    }

    fn detect_particle_collisions(&mut self) {
        for i in 0..self.particles.len() {
            for j in 0..self.particles.len() {
                if i != j && Self::detect_overlap(&self.particles[i], &self.particles[j]) {
                    self.particles[i].velocity[0] *= -1.0;
                    self.particles[i].velocity[1] *= -1.0;
                }
            }
        }
    }

    fn detect_overlap(particle1: &Particle, particle2: &Particle) -> bool {
        let delta_x = particle1.position.x - particle2.position.x;
        let delta_y = particle1.position.y - particle2.position.y;
        let dist = f32::sqrt(delta_x * delta_x + delta_y * delta_y);
        dist <= particle1.radius + particle2.radius
    }
}
