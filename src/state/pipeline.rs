use super::device;
use super::particle;
use super::simulation;
use super::vertex;
use rapier2d::prelude::*;
use wgpu::util::DeviceExt;

pub struct Pipeline {
    num_particles: u32,
    num_indices: u32,
    particle_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    fn create_buffers(
        device: &device::Device,
        particles: &Vec<particle::Particle>,
        vertices: &Vec<vertex::Vertex>,
        indices: &Vec<u32>,
    ) -> (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer) {
        (
            device
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Particle Buffer"),
                    contents: bytemuck::cast_slice(&particles),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                }),
            device
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                }),
            device
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(indices),
                    usage: wgpu::BufferUsages::INDEX,
                }),
        )
    }

    fn create_render_pipeline(device: &device::Device) -> wgpu::RenderPipeline {
        let render_shader = device
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Render Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });
        let render_pipeline_layout =
            device
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });
        let vertex_buffer_layout = &[
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<particle::Particle>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<vertex::Vertex>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![2 => Float32x3, 3 => Float32x3],
            },
        ];
        let render_pipeline =
            device
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
                        targets: &[Some(device.surface_format.into())],
                    }),
                    primitive: wgpu::PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState::default(),
                    multiview: None,
                });
        render_pipeline
    }

    pub fn new(
        particles: &Vec<particle::Particle>,
        vertices: &Vec<vertex::Vertex>,
        indices: &Vec<u32>,
        device: &device::Device,
    ) -> Self {
        let (particle_buffer, vertex_buffer, index_buffer) =
            Pipeline::create_buffers(&device, &particles, &vertices, &indices);
        let render_pipeline = Pipeline::create_render_pipeline(&device);

        Self {
            num_particles: particles.len() as u32,
            num_indices: indices.len() as u32,
            particle_buffer,
            vertex_buffer,
            index_buffer,
            render_pipeline,
        }
    }

    pub fn update(
        &self,
        particles: &mut Vec<particle::Particle>,
        device: &device::Device,
        simulation: &mut simulation::Simulation,
    ) {
        simulation.physics_pipeline.step(
            &vector![0.0, simulation.gravity],
            &simulation.integration_parameters,
            &mut simulation.island_manager,
            &mut simulation.broad_phase,
            &mut simulation.narrow_phase,
            &mut simulation.rigid_body_set,
            &mut simulation.collider_set,
            &mut simulation.impulse_joint_set,
            &mut simulation.multibody_joint_set,
            &mut simulation.ccd_solver,
            None,
            &(),
            &(),
        );

        for i in 0..simulation.rigid_body_handles.len() {
            let rigid_body = &simulation.rigid_body_set[simulation.rigid_body_handles[i]];
            particles[i].position[0] = rigid_body.translation().x;
            particles[i].position[1] = rigid_body.translation().y;
            println!("Ball altitude: {}", rigid_body.translation().y);
        }

        device
            .queue
            .write_buffer(&self.particle_buffer, 0, bytemuck::cast_slice(&particles));
    }

    pub fn render(&self, device: &device::Device) -> Result<(), wgpu::SurfaceError> {
        let frame = device.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = device
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
            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.num_particles);
        }

        device.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
        Ok(())
    }
}
