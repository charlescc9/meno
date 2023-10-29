use super::device;
use super::simulation;
use crate::state::camera::Camera;
use std::f32::consts::TAU;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ParticleRaw {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexRaw {
    pub offset: [f32; 3],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraMatrix {
    pub matrix: [[f32; 4]; 4],
}

impl CameraMatrix {
    fn new() -> Self {
        Self {
            matrix: glm::Mat4::identity().into(),
        }
    }

    fn update_camera_matrix(&mut self, camera: &Camera) {
        self.matrix = camera.generate_camera_matrix().into();
    }
}

pub struct Pipeline {
    num_particles: u32,
    num_indices: u32,
    max_velocity: f32,
    camera_matrix: CameraMatrix,
    particles_raw: Vec<ParticleRaw>,
    particle_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    simulation: simulation::Simulation,
}

impl Pipeline {
    pub fn generate_shader_particles(
        max_velocity: f32,
        particles: &Vec<simulation::Particle>,
        particles_raw: &mut Vec<ParticleRaw>,
    ) {
        let particle_positions: Vec<[f32; 3]> = particles
            .iter()
            .map(|p| p.position.as_slice().try_into().unwrap())
            .collect();
        let particles_colors: Vec<[f32; 3]> = particles
            .iter()
            .map(|p| {
                [
                    1.0 - p.velocity.magnitude() / max_velocity,
                    0.0,
                    f32::min(p.velocity.magnitude() / max_velocity, 1.0),
                ]
            })
            .collect();
        for i in 0..particles.len() {
            if particles_raw.len() > i {
                particles_raw[i].position = particle_positions[i];
                particles_raw[i].color = particles_colors[i];
            } else {
                particles_raw.push(ParticleRaw {
                    position: particle_positions[i],
                    color: particles_colors[i],
                })
            }
        }
    }

    pub fn generate_shader_vertices(num_sides: u32, radius: f32) -> (Vec<VertexRaw>, Vec<u32>) {
        let mut vertices = vec![VertexRaw {
            offset: [0.0, 0.0, 0.0],
        }];
        let mut indices = Vec::new();

        for i in 1..num_sides + 1 {
            let x = radius * f32::cos(i as f32 * TAU / num_sides as f32);
            let y = radius * f32::sin(i as f32 * TAU / num_sides as f32);
            vertices.push(VertexRaw {
                offset: [x, y, 0.0],
            });
            indices.extend([0, i, (i % num_sides) + 1]);
        }

        (vertices, indices)
    }

    fn create_buffers(
        max_velocity: f32,
        device: &device::Device,
        camera_matrix: &CameraMatrix,
        particles: &Vec<simulation::Particle>,
        particles_raw: &mut Vec<ParticleRaw>,
        vertices: &Vec<VertexRaw>,
        indices: &Vec<u32>,
    ) -> (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, wgpu::Buffer) {
        Pipeline::generate_shader_particles(max_velocity, particles, particles_raw);
        (
            device
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Particle Buffer"),
                    contents: bytemuck::cast_slice(particles_raw),
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
            device
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Camera Buffer"),
                    contents: bytemuck::cast_slice(&[camera_matrix.matrix]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                }),
        )
    }

    fn create_camera_bind_group(
        device: &device::Device,
        camera_buffer: &wgpu::Buffer,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let camera_bind_group_layout =
            device
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: Some("Camera Bind Group Layout"),
                });

        let camera_bind_group = device.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("Camera Bind Group"),
        });

        (camera_bind_group_layout, camera_bind_group)
    }

    fn create_render_pipeline(
        device: &device::Device,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
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
                    bind_group_layouts: &[&camera_bind_group_layout],
                    push_constant_ranges: &[],
                });
        let vertex_buffer_layout = &[
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<ParticleRaw>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<VertexRaw>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![2 => Float32x3],
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
        max_velocity: f32,
        simulation: simulation::Simulation,
        vertices: &Vec<VertexRaw>,
        indices: &Vec<u32>,
        device: &device::Device,
    ) -> Self {
        let mut particles_raw = Vec::new();

        let mut camera_matrix = CameraMatrix::new();
        camera_matrix.update_camera_matrix(&device.camera);
        let (particle_buffer, vertex_buffer, index_buffer, camera_buffer) =
            Pipeline::create_buffers(
                max_velocity,
                &device,
                &camera_matrix,
                &simulation.particles,
                &mut particles_raw,
                &vertices,
                &indices,
            );
        let (camera_bind_group_layout, camera_bind_group) =
            Pipeline::create_camera_bind_group(&device, &camera_buffer);

        let render_pipeline = Pipeline::create_render_pipeline(&device, &camera_bind_group_layout);

        Self {
            num_particles: simulation.particles.len() as u32,
            num_indices: indices.len() as u32,
            max_velocity,
            camera_matrix,
            particles_raw,
            particle_buffer,
            vertex_buffer,
            index_buffer,
            camera_buffer,
            camera_bind_group,
            render_pipeline,
            simulation,
        }
    }

    pub fn update(&mut self, device: &mut device::Device) {
        self.simulation.step();

        Pipeline::generate_shader_particles(
            self.max_velocity,
            &self.simulation.particles,
            &mut self.particles_raw,
        );
        device.queue.write_buffer(
            &self.particle_buffer,
            0,
            bytemuck::cast_slice(&self.particles_raw),
        );
        device.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&self.camera_matrix.matrix),
        );

        device.update_camera();
        self.camera_matrix.update_camera_matrix(&device.camera);
        device.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_matrix.matrix]),
        )
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
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
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
