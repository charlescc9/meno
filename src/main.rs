extern crate nalgebra_glm as glm;
use clap::Parser;
mod state;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'p', long, default_value_t = 10)]
    num_particles: u32,

    #[arg(short = 's', long, default_value_t = 64)]
    num_sides: u32,

    #[arg(short = 'n', long, default_value_t = 0.25)]
    min_mass: f32,

    #[arg(short = 'm', long, default_value_t = 1.0)]
    max_mass: f32,

    #[arg(short = 'v', long, default_value_t = 0.015)]
    max_velocity: f32,

    #[arg(short = 'r', long, default_value_t = 0.1)]
    radius: f32,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let state = state::State::new(
        args.num_particles,
        args.num_sides,
        args.min_mass,
        args.max_mass,
        args.max_velocity,
        args.radius,
    )
    .await;
    state.run().await;
}
