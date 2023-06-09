use clap::Parser;
mod state;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'n', long, default_value_t = 10)]
    num_particles: u32,

    #[arg(short = 'r', long, default_value_t = 0.1)]
    particle_radius: f32,

    #[arg(short = 's', long, default_value_t = 64)]
    particle_sides: u32,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let state = state::State::new(
        args.num_particles,
        args.particle_radius,
        args.particle_sides,
    )
    .await;
    state.run().await;
}
