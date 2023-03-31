use crate::components::Components;
use crate::particle::Particle;

pub fn get_euclidean_distance(point1: &Components, point2: &Components) -> f64 {
    ((point1.x - point2.x).powi(2) + (point1.y - point2.y).powi(2)).sqrt()
}

pub fn get_gravitational_force(
    particles: &Vec<Particle>,
    source_index: usize,
    g: f64,
) -> Components {
    let mut forces: Vec<Components> = Vec::new();
    let source = particles.get(source_index).unwrap();

    for i in 0..particles.len() {
        if i != source_index {
            let particle = particles.get(i).unwrap();
            let radius = get_euclidean_distance(&source.position, &particle.position);
            let distance_x = particle.position.x - source.position.x;
            let distance_y = particle.position.y - source.position.y;
            let force_x = (g * source.mass * particle.mass * distance_x) / radius.powi(2);
            let force_y = (g * source.mass * particle.mass * distance_y) / radius.powi(2);
            forces.push(Components {
                x: force_x,
                y: force_y,
            });
        }
    }

    Components {
        x: forces.iter().map(|f| f.x).sum(),
        y: forces.iter().map(|f| f.y).sum(),
    }
}

pub fn get_gravitational_potential_energy(particles: &Vec<Particle>, g: f64) -> f64 {
    // E_p = Sum(Gmm/r^2)
    let mut energy = 0.0;

    for i in 0..particles.len() - 1 {
        let particle1 = particles.get(i).unwrap();
        let particle2 = particles.get(i + 1).unwrap();
        let dist = get_euclidean_distance(&particle1.position, &particle2.position);
        energy += (g * particle1.mass * particle2.mass) / dist
    }

    energy
}

pub fn get_kinetic_energy(particles: &Vec<Particle>) -> f64 {
    // E_k = Sum(1/2mv^2)

    let energies: f64 = particles
        .iter()
        .map(|p| p.mass * (p.velocity.x.powi(2) + p.velocity.y.powi(2)))
        .sum();

    0.5 * energies
}
