use winit::event_loop::EventLoop;

pub struct Interface {
    pub window: winit::window::Window,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    pub surface_format: wgpu::TextureFormat,
    pub camera: Camera,
    pub camera_controller: CameraController,
}

pub struct Camera {
    eye: glm::Vec3,
    center: glm::Vec3,
    up: glm::Vec3,
    aspect: f32,
    fovy: f32,
    z_near: f32,
    z_far: f32,
}

pub struct CameraController {
    speed: f32,
    pub is_forward_pressed: bool,
    pub is_backward_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
}

impl Interface {
    pub async fn new(event_loop: &EventLoop<()>) -> Self {
        // Initialize winit window
        let window = winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap();

        // Initialize camera
        let camera = Camera {
            eye: glm::vec3(0.0, 1.0, 2.0),
            center: glm::vec3(0.0, 0.0, 0.0),
            up: glm::vec3(0.0, 1.0, 0.0),
            aspect: window.inner_size().width as f32 / window.inner_size().height as f32,
            fovy: 45.0,
            z_near: 0.1,
            z_far: 100.0,
        };
        let camera_controller = CameraController {
            speed: 0.2,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        };

        // Initialize surface
        let instance = wgpu::Instance::default();
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        // Initialize adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        // Initialize device
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        // Configure surface
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities.formats[0];
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: *surface_capabilities.present_modes.get(0).unwrap(),
            alpha_mode: *surface_capabilities.alpha_modes.get(0).unwrap(),
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);

        Self {
            window,
            device,
            queue,
            surface,
            surface_config,
            surface_format,
            camera,
            camera_controller,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    pub fn update_camera(&mut self) {
        let forward = self.camera.center - self.camera.eye;
        if self.camera_controller.is_forward_pressed
            && forward.magnitude() > self.camera_controller.speed
        {
            self.camera.eye += forward.normalize() * self.camera_controller.speed;
        }
        if self.camera_controller.is_backward_pressed {
            self.camera.eye -= forward.normalize() * self.camera_controller.speed;
        }

        let right = forward.normalize().cross(&self.camera.up);
        let forward = self.camera.center - self.camera.eye;
        let forward_mag = forward.magnitude();
        if self.camera_controller.is_right_pressed {
            self.camera.eye = self.camera.center
                - (forward + right * self.camera_controller.speed).normalize() * forward_mag;
        }
        if self.camera_controller.is_left_pressed {
            self.camera.eye = self.camera.center
                - (forward - right * self.camera_controller.speed).normalize() * forward_mag;
        }
    }
}

impl Camera {
    pub fn to_shader(&self) -> glm::Mat4 {
        #[rustfmt::skip]
            let opengl_to_wgpu = glm::mat4(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.5, 0.5,
            0.0, 0.0, 0.0, 1.0,
        );

        let view = glm::look_at_rh(&self.eye, &self.center, &self.up);
        let projection = glm::perspective(self.aspect, self.fovy, self.z_near, self.z_far);
        return opengl_to_wgpu * projection * view;
    }
}
