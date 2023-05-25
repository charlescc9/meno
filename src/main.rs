use clap::Parser;
use meno::ParticleState;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 't', long, default_value_t = 16)]
    height: u32,

    #[arg(short, long, default_value_t = 16)]
    width: u32,

    #[arg(short, long, default_value_t = 10)]
    num_particles: u32,

    #[arg(short, long, default_value_t = 10.0)]
    max_mass: f64,

    #[arg(short = 's', long, default_value_t = 1.0)]
    max_speed: f64,
}

pub async fn run(event_loop: winit::event_loop::EventLoop<()>, mut state: ParticleState) {
    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window.id() => match event {
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
                state.resize(*physical_size);
            }
            _ => {}
        },
        winit::event::Event::RedrawRequested(window_id) if window_id == state.window.id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => state.resize(state.window.inner_size()),
                Err(wgpu::SurfaceError::OutOfMemory) => {
                    *control_flow = winit::event_loop::ControlFlow::Exit
                }
                Err(e) => eprintln!("{:?}", e),
            }
        }
        winit::event::Event::MainEventsCleared => {
            state.window.request_redraw();
        }
        _ => {}
    });
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Emergence")
        .build(&event_loop)
        .unwrap();

    let state = ParticleState::new(window, args.num_particles).await;
    run(event_loop, state).await;
}
