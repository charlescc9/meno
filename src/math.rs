use rand::prelude::*;
use std::f64::consts::PI;

use crate::particle::Particle;
use crate::point::Point;
use crate::vector::Vector;

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

pub fn get_direction_between_points(source: &Point, destination: &Point) -> Point {
    // Gets direction from src to dst, as if dst is the origin
    Point { x: source.x - destination.x, y: source.y - destination.y }
}

pub fn get_gravitational_force(
    particles: &Vec<Particle>,
    source_idx: usize,
    gravity: f64,
) -> Vector {
    let mut forces: Vec<Vector> = Vec::new();
    let source = particles.get(source_idx).unwrap();

    for i in 0..particles.len() {
        if i != source_idx {
            let particle = particles.get(i).unwrap();
            let radius = get_euclidean_distance(&source.position, &particle.position);
            let force = (gravity * source.mass * particle.mass) / radius.powi(2);
            let direction = get_direction_between_points(&source.position, &particle.position);
            forces.push(Vector { magnitude: force, direction });
        }
    }

    // Todo: fix this
    return forces.pop().unwrap();
}
