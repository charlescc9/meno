mod camera;
mod device;
mod pipeline;
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
        let (vertices, indices) = pipeline::Pipeline::generate_shader_vertices(num_sides, radius);

        let camera = camera::Camera {
            eye: glm::vec3(0.0, 1.0, 2.0),
            target: glm::vec3(0.0, 0.0, 0.0),
            up: glm::vec3(0.0, 1.0, 0.0),
            aspect: window.inner_size().width as f32 / window.inner_size().height as f32,
            fov: 45.0,
            z_near: 0.1,
            z_far: 100.0,
        };
        let camera_controller = camera::CameraController::new(0.2);
        let device = device::Device::new(&window, camera, camera_controller).await;

        let pipeline = pipeline::Pipeline::new(
            max_velocity,
            simulation,
            &vertices,
            &indices,
            &device,
        );

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
                                self.device.camera_controller.is_forward_pressed = is_pressed;
                            }
                            winit::event::VirtualKeyCode::A => {
                                self.device.camera_controller.is_left_pressed = is_pressed;
                            }
                            winit::event::VirtualKeyCode::S => {
                                self.device.camera_controller.is_backward_pressed = is_pressed;
                            }
                            winit::event::VirtualKeyCode::D => {
                                self.device.camera_controller.is_right_pressed = is_pressed;
                            }
                            _ => (),
                        }
                    }
                    _ => {}
                },
                winit::event::Event::RedrawRequested(window_id)
                    if window_id == self.window.id() =>
                {
                    self.pipeline.update(&mut self.device);
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
