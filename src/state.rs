mod device;
mod particle;
mod pipeline;
mod simulation;
mod vertex;

pub struct State {
    event_loop: winit::event_loop::EventLoop<()>,
    window: winit::window::Window,
    particles: Vec<particle::Particle>,
    device: device::Device,
    pipeline: pipeline::Pipeline,
}

impl State {
    pub async fn new(num_particles: u32, particle_radius: f32, particle_sides: u32) -> Self {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap();
        let particles = particle::Particle::create_particles(num_particles);
        let (vertices, indices) =
            vertex::VertexRaw::create_particles_vertices(particle_radius, particle_sides);
        let device = device::Device::new(&window).await;
        let pipeline = pipeline::Pipeline::new(&particles, &vertices, &indices, &device);

        State {
            window,
            event_loop,
            particles,
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
                    self.pipeline.update(&mut self.particles, &self.device);
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
