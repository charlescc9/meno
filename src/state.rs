use pipeline::Pipeline;
use shader_types::VertexRaw;
use simulation::Simulation;
use wgpu::{
    Device, DeviceDescriptor, Features, Instance, Limits, MemoryHints, PowerPreference, Queue,
    RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages,
};
use winit::{dpi::PhysicalSize, window::Window};

mod pipeline;
mod shader_types;
mod simulation;

pub struct State<'a> {
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'a>,
    pub config: SurfaceConfiguration,
    pub window: &'a Window,
    pub pipeline: Pipeline,
    pub size: winit::dpi::PhysicalSize<u32>,
}

impl<'a> State<'a> {
    pub async fn new(
        num_particles: u32,
        num_sides: u32,
        min_mass: f32,
        max_mass: f32,
        max_velocity: f32,
        radius: f32,
        window: &'a Window,
    ) -> Self {
        let size = window.inner_size();
        let simulation = Simulation::new(num_particles, min_mass, max_mass, max_velocity, radius);
        let (vertices, indices) = VertexRaw::generate_shader_vertices(num_sides, radius);

        let instance = Instance::default();
        let surface = instance.create_surface(window).unwrap();
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("Device"),
                    required_features: Features::empty(),
                    required_limits: Limits::default(),
                    memory_hints: MemoryHints::default(),
                },
                None,
            )
            .await
            .unwrap();
        let surface_capabilities = surface.get_capabilities(&adapter);
        let format = surface_capabilities.formats[0];
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: *surface_capabilities.present_modes.get(0).unwrap(),
            alpha_mode: *surface_capabilities.alpha_modes.get(0).unwrap(),
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let pipeline = Pipeline::new(
            max_velocity,
            simulation,
            &vertices,
            &indices,
            &device,
            &format,
        );

        State {
            device,
            queue,
            surface,
            config,
            window,
            pipeline,
            size,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}
