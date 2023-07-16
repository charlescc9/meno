mod device;
mod pipeline;
mod shader_types;
mod simulation;

pub struct State {
    event_loop: winit::event_loop::EventLoop<()>,
    window: winit::window::Window,
    device: device::Device,
    pipeline: pipeline::Pipeline,
}

impl State {
    pub async fn new(
        num_particles: u32,
        num_sides: u32,
        min_mass: f32,
        max_mass: f32,
        max_velocity: f32,
        radius: f32,
    ) -> Self {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap();
        let simulation =
            simulation::Simulation::new(num_particles, min_mass, max_mass, max_velocity, radius);
        let (vertices, indices) =
            shader_types::VertexRaw::generate_shader_vertices(num_sides, radius);
        let device = device::Device::new(&window).await;
        let pipeline =
            pipeline::Pipeline::new(max_velocity, simulation, &vertices, &indices, &device);

        State {
            window,
            event_loop,
            device,
            pipeline,
        }
    }

    pub async fn run(mut self) {
        self.event_loop
            .run(move |event, _, control_flow| match event {
                winit::event::Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => match event {
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
                        self.device.resize(*physical_size);
                    }
                    _ => {}
                },
                winit::event::Event::RedrawRequested(window_id)
                    if window_id == self.window.id() =>
                {
                    self.pipeline.update(&self.device);
                    match self.pipeline.render(&self.device) {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => {
                            self.device.resize(self.window.inner_size())
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            *control_flow = winit::event_loop::ControlFlow::Exit
                        }
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                winit::event::Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                _ => {}
            });
    }
}
