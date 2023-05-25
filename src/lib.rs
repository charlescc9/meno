use rand::Rng;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Particle {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub offset: [f32; 3],
    pub color: [f32; 3],
}

const PARTICLE_VERTICES: &[Vertex] = &[
    Vertex {
        offset: [-0.05, 0.05, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        offset: [0.05, 0.05, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        offset: [0.05, -0.05, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        offset: [-0.05, -0.05, 0.0],
        color: [1.0, 0.0, 0.0],
    },
];

const PARTICLE_VERTICES_INDICES: &[u32] = &[1, 0, 2, 3, 2, 0];

pub struct WgpuState {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    surface_format: wgpu::TextureFormat,
}

pub struct ParticleState {
    particles: Vec<Particle>,
    pub window: winit::window::Window,
    frame_num: usize,
    num_particles: u32,
    wgpu_state: WgpuState,
    particle_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
}

impl ParticleState {
    fn create_particles(num_particles: u32) -> Vec<Particle> {
        let mut particles = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..num_particles {
            particles.push(Particle {
                position: [
                    rng.gen::<f32>() * 2.0 - 1.0,
                    rng.gen::<f32>() * 2.0 - 1.0,
                    0.0,
                ],
                velocity: [
                    (rng.gen::<f32>() * 2.0 - 1.0) * 0.1,
                    (rng.gen::<f32>() * 2.0 - 1.0) * 0.1,
                    0.0,
                ],
            })
        }
        particles
    }

    async fn init_wgpu(window: &winit::window::Window) -> WgpuState {
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

        WgpuState {
            device,
            queue,
            surface,
            surface_config,
            surface_format,
        }
    }

    fn create_buffers(
        wgpu_state: &WgpuState,
        particles: &Vec<Particle>,
    ) -> (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer) {
        (
            wgpu_state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Particle Buffer"),
                    contents: bytemuck::cast_slice(&particles),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                }),
            wgpu_state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(PARTICLE_VERTICES),
                    usage: wgpu::BufferUsages::VERTEX,
                }),
            wgpu_state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(&PARTICLE_VERTICES_INDICES),
                    usage: wgpu::BufferUsages::INDEX,
                }),
        )
    }

    fn create_render_pipeline(wgpu_state: &WgpuState) -> wgpu::RenderPipeline {
        let render_shader = wgpu_state
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Render Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("render.wgsl").into()),
            });
        let render_pipeline_layout =
            wgpu_state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });
        let vertex_buffer_layout = &[
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Particle>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![2 => Float32x3, 3 => Float32x3],
            },
        ];
        let render_pipeline =
            wgpu_state
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &render_shader,
                        entry_point: "main_vertex",
                        buffers: vertex_buffer_layout,
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &render_shader,
                        entry_point: "main_fragment",
                        targets: &[Some(wgpu_state.surface_format.into())],
                    }),
                    primitive: wgpu::PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState::default(),
                    multiview: None,
                });
        render_pipeline
    }

    pub async fn new(window: winit::window::Window, num_particles: u32) -> Self {
        let frame_num = 0;
        let wgpu_state = ParticleState::init_wgpu(&window).await;
        let particles = ParticleState::create_particles(num_particles);
        let (particle_buffer, vertex_buffer, index_buffer) =
            ParticleState::create_buffers(&wgpu_state, &particles);
        let render_pipeline = ParticleState::create_render_pipeline(&wgpu_state);

        Self {
            particles,
            window,
            frame_num,
            num_particles,
            wgpu_state,
            particle_buffer,
            vertex_buffer,
            index_buffer,
            render_pipeline,
        }
    }

    pub fn update(&mut self) {
        for i in 0..self.particles.len() {
            self.particles[i].position[0] += 0.01;
            if self.particles[i].position[0] > 1.0 {
                self.particles[i].position[0] -= 2.0;
            }
            self.particles[i].position[1] += 0.01;
            if self.particles[i].position[1] > 1.0 {
                self.particles[i].position[1] -= 2.0;
            }
        }
        self.wgpu_state.queue.write_buffer(
            &self.particle_buffer,
            0,
            bytemuck::cast_slice(&self.particles),
        )
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.wgpu_state.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =
            self.wgpu_state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.5,
                            g: 0.5,
                            b: 0.5,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.particle_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(
                0..PARTICLE_VERTICES_INDICES.len() as u32,
                0,
                0..self.num_particles,
            );
        }

        self.wgpu_state
            .queue
            .submit(std::iter::once(encoder.finish()));
        frame.present();
        self.frame_num += 1;
        Ok(())
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.wgpu_state.surface_config.width = new_size.width;
            self.wgpu_state.surface_config.height = new_size.height;
            self.wgpu_state
                .surface
                .configure(&self.wgpu_state.device, &self.wgpu_state.surface_config);
        }
    }
}
