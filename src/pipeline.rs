use super::interface;
use crate::simulation;
use interface::Interface;
use simulation::Simulation;
use std::f32::consts::TAU;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ShaderParticle {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ShaderVertex {
    pub offset: [f32; 3],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ShaderCameraMatrix {
    pub matrix: [[f32; 4]; 4],
}

pub struct Pipeline {
    pub interface: Interface,
    simulation: Simulation,
    num_particles: u32,
    num_indices: u32,
    max_velocity: f32,
    shader_particles: Vec<ShaderParticle>,
    shader_camera_matrix: ShaderCameraMatrix,
    particle_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
}

impl ShaderParticle {
    fn new() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            color: [0.0, 0.0, 0.0],
        }
    }

    fn update(&mut self, position: glm::Vec3, color: glm::Vec3) {
        self.position = position.into();
        self.color = color.into();
    }
}

impl ShaderCameraMatrix {
    fn new() -> Self {
        Self {
            matrix: glm::Mat4::identity().into(),
        }
    }

    fn update(&mut self, matrix: glm::Mat4) {
        self.matrix = matrix.into();
    }
}

impl Pipeline {
    pub fn new(
        interface: Interface,
        simulation: Simulation,
        max_velocity: f32,
        num_sides: u32,
        radius: f32,
    ) -> Self {
        let num_particles = simulation.particles.len();

        // Initialize shader vertices and indices
        let mut shader_vertices = vec![ShaderVertex {
            offset: [0.0, 0.0, 0.0],
        }];
        let mut shader_indices = Vec::new();
        for i in 1..num_sides + 1 {
            let x = radius * f32::cos(i as f32 * TAU / num_sides as f32);
            let y = radius * f32::sin(i as f32 * TAU / num_sides as f32);
            shader_vertices.push(ShaderVertex {
                offset: [x, y, 0.0],
            });
            shader_indices.extend([0, i, (i % num_sides) + 1]);
        }
        let num_indices = shader_indices.len() as u32;

        // Initialize shader particles
        let mut shader_particles = Vec::new();
        for i in 0..num_particles {
            let mut shader_particle = ShaderParticle::new();
            let (position, color) = simulation.particles[i].to_shader(max_velocity);
            shader_particle.update(position, color);
            shader_particles.push(shader_particle);
        }

        // Initialize shader camera matrix
        let mut shader_camera_matrix = ShaderCameraMatrix::new();
        shader_camera_matrix.update(interface.camera.to_shader());

        // Initialize buffers
        let (particle_buffer, vertex_buffer, index_buffer, camera_buffer) =
            Pipeline::create_buffers(
                &interface,
                &shader_vertices,
                &shader_indices,
                &shader_particles,
                &shader_camera_matrix,
            );

        // Initialize bind groups
        let (camera_bind_group_layout, camera_bind_group) =
            Pipeline::create_camera_bind_group(&interface, &camera_buffer);

        // initialize pipeline
        let render_pipeline =
            Pipeline::create_render_pipeline(&interface, &camera_bind_group_layout);

        Self {
            interface,
            simulation,
            num_particles: num_particles as u32,
            num_indices,
            max_velocity,
            shader_particles,
            shader_camera_matrix,
            particle_buffer,
            vertex_buffer,
            index_buffer,
            camera_buffer,
            camera_bind_group,
            render_pipeline,
        }
    }

    fn create_buffers(
        interface: &Interface,
        shader_vertices: &Vec<ShaderVertex>,
        shader_indices: &Vec<u32>,
        shader_particles: &Vec<ShaderParticle>,
        shader_camera_matrix: &ShaderCameraMatrix,
    ) -> (wgpu::Buffer, wgpu::Buffer, wgpu::Buffer, wgpu::Buffer) {
        (
            interface
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Particle Buffer"),
                    contents: bytemuck::cast_slice(shader_particles),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                }),
            interface
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(shader_vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                }),
            interface
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(shader_indices),
                    usage: wgpu::BufferUsages::INDEX,
                }),
            interface
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Camera Buffer"),
                    contents: bytemuck::cast_slice(&[shader_camera_matrix.matrix]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                }),
        )
    }

    fn create_camera_bind_group(
        interface: &Interface,
        camera_buffer: &wgpu::Buffer,
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let camera_bind_group_layout =
            interface
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

        let camera_bind_group = interface
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
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
        interface: &Interface,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let render_shader = interface
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Render Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });
        let render_pipeline_layout =
            interface
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&camera_bind_group_layout],
                    push_constant_ranges: &[],
                });
        let vertex_buffer_layout = &[
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<ShaderParticle>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Instance,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
            },
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<ShaderVertex>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![2 => Float32x3],
            },
        ];
        let render_pipeline =
            interface
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
                        targets: &[Some(interface.surface_format.into())],
                    }),
                    primitive: wgpu::PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState::default(),
                    multiview: None,
                });
        render_pipeline
    }

    pub fn update(&mut self) {
        // Update simulation
        self.simulation.step();

        // Update particles
        for i in 0..self.shader_particles.len() {
            let (position, color) = self.simulation.particles[i].to_shader(self.max_velocity);
            self.shader_particles[i].update(position, color);
        }
        self.interface.queue.write_buffer(
            &self.particle_buffer,
            0,
            bytemuck::cast_slice(&self.shader_particles),
        );

        // Update camera
        self.interface.update_camera();
        self.shader_camera_matrix
            .update(self.interface.camera.to_shader());
        self.interface.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&self.shader_camera_matrix.matrix),
        );
    }

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.interface.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =
            self.interface
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

        self.interface
            .queue
            .submit(std::iter::once(encoder.finish()));
        frame.present();
        Ok(())
    }
}
