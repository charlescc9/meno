extern crate nalgebra_glm as glm;
use clap::Parser;
use state::State;
use tracing_subscriber::fmt;
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};
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

    #[arg(short = 'v', long, default_value_t = 0.02)]
    max_velocity: f32,

    #[arg(short = 'r', long, default_value_t = 0.1)]
    radius: f32,
}

pub async fn run(
    num_particles: u32,
    num_sides: u32,
    min_mass: f32,
    max_mass: f32,
    max_velocity: f32,
    radius: f32,
) {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(
        num_particles,
        num_sides,
        min_mass,
        max_mass,
        max_velocity,
        radius,
        &window,
    )
    .await;

    let _ = event_loop.run(move |event, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => control_flow.exit(),
            WindowEvent::Resized(physical_size) => {
                state.resize(*physical_size);
            }
            WindowEvent::RedrawRequested => {
                state.window.request_redraw();
                state.pipeline.update(&state.queue);
                match state
                    .pipeline
                    .render(&state.device, &state.surface, &state.queue)
                {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        state.resize(state.size)
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => control_flow.exit(),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => {}
        },
        Event::AboutToWait => {
            state.window.request_redraw();
        }
        _ => {}
    });
}

#[tokio::main]
async fn main() {
    fmt::init();
    let args = Args::parse();
    run(
        args.num_particles,
        args.num_sides,
        args.min_mass,
        args.max_mass,
        args.max_velocity,
        args.radius,
    )
    .await;
}
