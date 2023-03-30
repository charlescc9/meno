mod math;
mod particle;
mod components;
mod space;

use clap::Parser;
use std::{f64, thread, time};

use particle::Particle;
use space::Space;

const G: f64 = 6.67430e-11f64;

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
}

fn main() {
    // Todo: Compute totoal momentum and ensure that it is conserved

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
        time: 0,
    };
    println!("{}", space);

    // Update particles
    loop {
        for i in 0..space.particles.len() {
            let gravitational_force = math::get_gravitational_force(&space.particles, i, G);

            space.particles.get_mut(i).unwrap().update(
                args.width,
                args.height,
                gravitational_force,
            );
        }

        println!("{}", space);
        space.time += 1;
        thread::sleep(time::Duration::from_millis(1000));
    }
}
