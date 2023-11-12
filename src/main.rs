extern crate nalgebra_glm as glm;
use clap::Parser;
use interface::Interface;
use pipeline::Pipeline;
use winit::event::Event;
use winit::event_loop::EventLoop;

mod interface;
mod pipeline;
mod simulation;

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

pub async fn init(
    num_particles: u32,
    num_sides: u32,
    min_mass: f32,
    max_mass: f32,
    max_velocity: f32,
    radius: f32,
) -> (EventLoop<()>, Pipeline) {
    // Initialize event loop
    let event_loop = winit::event_loop::EventLoop::new();

    // Initialize interface
    let interface = Interface::new(&event_loop).await;

    // Initialize simulation
    let simulation =
        simulation::Simulation::new(num_particles, min_mass, max_mass, max_velocity, radius);

    // Initialize render pipeline
    let pipeline = Pipeline::new(interface, simulation, max_velocity, num_sides, radius);

    (event_loop, pipeline)
}

pub async fn run(event_loop: EventLoop<()>, mut pipeline: Pipeline) {
    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == pipeline.interface.window.id() => match event {
            winit::event::WindowEvent::CloseRequested
            | winit::event::WindowEvent::KeyboardInput {
                input:
                    winit::event::KeyboardInput {
                        state: winit::event::ElementState::Pressed,
                        virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = winit::event_loop::ControlFlow::Exit,
            winit::event::WindowEvent::Resized(physical_size) => {
                pipeline.interface.resize(*physical_size);
            }
            winit::event::WindowEvent::KeyboardInput {
                input:
                    winit::event::KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == winit::event::ElementState::Pressed;
                match keycode {
                    winit::event::VirtualKeyCode::W => {
                        pipeline.interface.camera_controller.is_forward_pressed = is_pressed;
                    }
                    winit::event::VirtualKeyCode::A => {
                        pipeline.interface.camera_controller.is_left_pressed = is_pressed;
                    }
                    winit::event::VirtualKeyCode::S => {
                        pipeline.interface.camera_controller.is_backward_pressed = is_pressed;
                    }
                    winit::event::VirtualKeyCode::D => {
                        pipeline.interface.camera_controller.is_right_pressed = is_pressed;
                    }
                    _ => (),
                }
            }
            _ => {}
        },
        Event::RedrawRequested(window_id) if window_id == pipeline.interface.window.id() => {
            pipeline.update();
            match pipeline.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => pipeline
                    .interface
                    .resize(pipeline.interface.window.inner_size()),
                Err(wgpu::SurfaceError::OutOfMemory) => {
                    *control_flow = winit::event_loop::ControlFlow::Exit
                }
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            pipeline.interface.window.request_redraw();
        }
        _ => {}
    });
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let (event_loop, pipeline) = init(
        args.num_particles,
        args.num_sides,
        args.min_mass,
        args.max_mass,
        args.max_velocity,
        args.radius,
    )
    .await;
    run(event_loop, pipeline).await;
}
