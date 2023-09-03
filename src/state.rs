mod camera;
mod device;
mod pipeline;
mod simulation;

struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }
}

pub struct State {
    event_loop: winit::event_loop::EventLoop<()>,
    window: winit::window::Window,
    device: device::Device,
    camera: camera::Camera,
    camera_controller: CameraController,
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
        let device = device::Device::new(&window).await;
        let camera = camera::Camera {
            eye: glm::vec3(0.0, 1.0, 2.0),
            target: glm::vec3(0.0, 0.0, 0.0),
            up: glm::vec3(0.0, 1.0, 0.0),
            aspect: window.inner_size().width as f32 / window.inner_size().height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };
        let camera_controller = CameraController::new(0.2);
        let pipeline = pipeline::Pipeline::new(
            max_velocity,
            simulation,
            &vertices,
            &indices,
            &device,
            &camera,
        );

        State {
            window,
            event_loop,
            device,
            camera,
            camera_controller,
            pipeline,
        }
    }

    fn update_camera(&self, camera: &mut camera::Camera) {
        let forward = camera.target - camera.eye;

        if self.camera_controller.is_forward_pressed && 
        forward.magnitude() > self.camera_controller.speed {
            camera.eye += forward.normalize() * self.camera_controller.speed;
        }
        if self.camera_controller.is_backward_pressed {
            camera.eye -= forward.normalize() * self.camera_controller.speed;
        }

        let right = forward.normalize().cross(&camera.up);

        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.camera_controller.is_right_pressed {
            camera.eye = camera.target - 
            (forward + right * self.camera_controller.speed).normalize() * forward_mag;
        }
        if self.camera_controller.is_left_pressed {
            camera.eye = camera.target - 
            (forward - right * self.camera_controller.speed).normalize() * forward_mag;
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
                        input: winit::event::KeyboardInput {
                            state,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                        ..
                    } => {
                        let is_pressed = *state == winit::event::ElementState::Pressed;
                        match keycode {
                            winit::event::VirtualKeyCode::W => {
                                self.camera_controller.is_forward_pressed = is_pressed;
                            }
                            winit::event::VirtualKeyCode::A => {
                                self.camera_controller.is_left_pressed = is_pressed;
                            }
                            winit::event::VirtualKeyCode::S => {
                                self.camera_controller.is_backward_pressed = is_pressed;
                            }
                            winit::event::VirtualKeyCode::D => {
                                self.camera_controller.is_right_pressed = is_pressed;
                            }
                            _ => (),
                        }
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
