mod math;
mod particle;
mod point;
mod space;
mod vector;

use clap::Parser;
use std::{f64, thread, time};

use particle::Particle;
use space::Space;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 't', long, default_value_t = 16)]
    height: u32,

    #[arg(short, long, default_value_t = 16)]
    width: u32,

    #[arg(short, long, default_value_t = 10)]
    num_partiles: u32,

    #[arg(short, long, default_value_t = 10.0)]
    max_mass: f64,

    #[arg(short = 's', long, default_value_t = 1.0)]
    max_speed: f64,

    #[arg(short, long, default_value_t = 9.81)]
    gravity: f64,
}

fn main() {
    let args = Args::parse();
    println!("Running Meno with the following {:?}", args);

    // Create particles
    let mut particles: Vec<Particle> = Vec::new();
    for i in 0..args.num_partiles {
        let particle = Particle::new(
            &mut rand::thread_rng(),
            i,
            args.width,
            args.height,
            args.max_mass,
            args.max_speed,
        );
        particles.push(particle);
    }

    // Initialize space
    let mut space = Space {
        height: args.height,
        width: args.width,
        particles: particles,
    };
    println!("{}", space);

    loop {
        // Update particles
        for i in 0..space.particles.len() {
            let gravitational_potential =
                math::get_gravitational_potential(&space.particles, i, args.gravity);

            println!("Gravitational potential for particle {}: {}", i, gravitational_potential);

            space.particles.get_mut(i).unwrap().update(
                args.width,
                args.height,
                gravitational_potential,
            );
        }

        println!("{}", space);
        thread::sleep(time::Duration::from_millis(1000));
    }
}
