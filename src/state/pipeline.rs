use wgpu::util::DeviceExt;
use wgpu::BlendState;
use wgpu::ColorTargetState;
use wgpu::ColorWrites;
use wgpu::Device;
use wgpu::FragmentState;
use wgpu::PipelineCompilationOptions;
use wgpu::Queue;
use wgpu::RenderPassDescriptor;
use wgpu::RenderPipelineDescriptor;
use wgpu::StoreOp;
use wgpu::Surface;
use wgpu::TextureFormat;
use wgpu::VertexState;

use super::shader_types;
use super::simulation;

pub struct Pipeline {
    num_particles: u32,
    num_indices: u32,
    max_velocity: f32,
    particles_raw: Vec<shader_types::ParticleRaw>,
    particle_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
    simulation: simulation::Simulation,
}

impl Pipeline {
    fn create_buffers(
        max_velocity: f32,
        device: &Device,
        particles: &Vec<simulation::Particle>,
        particles_raw: &mut Vec<shader_types::ParticleRaw>,
        vertices: &Vec<shader_types::VertexRaw>,
        indices: &Vec<u32>,
    ) -> (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer) {
        shader_types::ParticleRaw::generate_shader_particles(
            max_velocity,
            particles,
            particles_raw,
        );
        (
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Particle Buffer"),
                contents: bytemuck::cast_slice(particles_raw),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }),
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
        )
    }

    fn create_render_pipeline(device: &Device, format: &TextureFormat) -> wgpu::RenderPipeline {
        let render_shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        let vertex_buffer_layout = &[
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<shader_types::ParticleRaw>()
                    as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<shader_types::VertexRaw>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![2 => Float32x3],
            },
        ];
        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &render_shader,
                entry_point: "main_vertex",
                buffers: vertex_buffer_layout,
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &render_shader,
                entry_point: "main_fragment",
                targets: &[Some(ColorTargetState {
                    format: *format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        render_pipeline
    }

    pub fn new(
        max_velocity: f32,
        simulation: simulation::Simulation,
        vertices: &Vec<shader_types::VertexRaw>,
        indices: &Vec<u32>,
        device: &Device,
        format: &TextureFormat,
    ) -> Self {
        let mut particles_raw = Vec::new();
        let (particle_buffer, vertex_buffer, index_buffer) = Pipeline::create_buffers(
            max_velocity,
            &device,
            &simulation.particles,
            &mut particles_raw,
            &vertices,
            &indices,
        );
        let render_pipeline = Pipeline::create_render_pipeline(device, format);

        Self {
            num_particles: simulation.particles.len() as u32,
            num_indices: indices.len() as u32,
            max_velocity,
            particles_raw,
            particle_buffer,
            vertex_buffer,
            index_buffer,
            render_pipeline,
            simulation,
        }
    }

    pub fn update(&mut self, queue: &Queue) {
        self.simulation.step();

        shader_types::ParticleRaw::generate_shader_particles(
            self.max_velocity,
            &self.simulation.particles,
            &mut self.particles_raw,
        );
        queue.write_buffer(
            &self.particle_buffer,
            0,
            bytemuck::cast_slice(&self.particles_raw),
        );
    }

    pub fn render(
        &self,
        device: &Device,
        surface: &Surface,
        queue: &Queue,
    ) -> Result<(), wgpu::SurfaceError> {
        let frame = surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.particle_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.num_particles);
        }

        queue.submit(std::iter::once(encoder.finish()));
        frame.present();
        Ok(())
    }
}
