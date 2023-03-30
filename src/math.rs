use crate::particle::Particle;
use crate::components::Components;


pub fn get_euclidean_distance(point1: &Components, point2: &Components) -> f64 {
    ((point1.x - point2.x).powi(2) + (point1.y - point2.y).powi(2)).sqrt()
}

pub fn get_gravitational_force(
    particles: &Vec<Particle>,
    source_idx: usize,
    gravity: f64,
) -> Components {
    let mut forces: Vec<Components> = Vec::new();
    let source = particles.get(source_idx).unwrap();

    for i in 0..particles.len() {
        if i != source_idx {
            let particle = particles.get(i).unwrap();
            let radius = get_euclidean_distance(&source.position, &particle.position);
            let distance_x = particle.position.x - source.position.x;
            let distance_y = particle.position.y - source.position.y;
            let force_x = (gravity * source.mass * particle.mass * distance_x) / radius.powi(2);
            let force_y = (gravity * source.mass * particle.mass * distance_y) / radius.powi(2);
            forces.push(Components { x: force_x, y: force_y });
        }
    }

    Components { x: forces.iter().map(|f| f.x).sum(), y: forces.iter().map(|f| f.y).sum() }
}
