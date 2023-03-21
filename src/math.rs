use rand::prelude::*;
use std::f64::consts::PI;

use crate::particle::Particle;
use crate::point::Point;

pub fn get_random_direction(gen: &mut ThreadRng) -> Point {
    let theta: f64 = 2.0 * PI * gen.gen_range(0.0..1.0);
    Point {
        x: theta.sin(),
        y: theta.cos(),
    }
}

pub fn get_euclidean_distance(point1: &Point, point2: &Point) -> f64 {
    return ((point1.x - point2.x).powi(2) + (point1.y - point2.y).powi(2)).sqrt();
}

pub fn get_gravitational_potential(
    particles: &Vec<Particle>,
    source_idx: usize,
    gravity: f64,
) -> f64 {
    let mut potentials: Vec<f64> = Vec::new();
    let source_particle = particles.get(source_idx).unwrap();

    for i in 0..particles.len() {
        let particle = particles.get(i).unwrap();
        let radius = get_euclidean_distance(&source_particle.position, &particle.position);
        let potential = (gravity * source_particle.mass) / radius.powi(2);
        potentials.push(potential);
    }

    return potentials.iter().sum();
}
