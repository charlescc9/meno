use crate::state::camera::{Camera, CameraController};

pub struct Device {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub surface_format: wgpu::TextureFormat,
    pub camera: Camera,
    pub camera_controller: CameraController,
}

impl Device {
    pub async fn new(
        window: &winit::window::Window,
        camera: Camera,
        camera_controller: CameraController,
    ) -> Self {
        let instance = wgpu::Instance::default();
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
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

        Device {
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
        let forward = self.camera.target - self.camera.eye;

        if self.camera_controller.is_forward_pressed
            && forward.magnitude() > self.camera_controller.speed
        {
            self.camera.eye += forward.normalize() * self.camera_controller.speed;
        }
        if self.camera_controller.is_backward_pressed {
            self.camera.eye -= forward.normalize() * self.camera_controller.speed;
        }

        let right = forward.normalize().cross(&self.camera.up);

        let forward = self.camera.target - self.camera.eye;
        let forward_mag = forward.magnitude();

        if self.camera_controller.is_right_pressed {
            self.camera.eye = self.camera.target
                - (forward + right * self.camera_controller.speed).normalize() * forward_mag;
        }
        if self.camera_controller.is_left_pressed {
            self.camera.eye = self.camera.target
                - (forward - right * self.camera_controller.speed).normalize() * forward_mag;
        }
    }
}
