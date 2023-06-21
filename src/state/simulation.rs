use super::particle;

pub fn step(particles: &mut Vec<particle::Particle>) {
    for i in 0..particles.len() {
        particles[i].position[0] += 0.01;
        if particles[i].position[0] > 1.0 {
            particles[i].position[0] -= 2.0;
        }
        particles[i].position[1] += 0.01;
        if particles[i].position[1] > 1.0 {
            particles[i].position[1] -= 2.0;
        }
    }
}
