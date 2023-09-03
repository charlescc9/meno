struct Camera {
    view_projection: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: Camera;

struct Particle {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexInput {
    @location(2) offset: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>
}

@vertex
fn main_vertex(particle: Particle, input_vertex: VertexInput) -> VertexOutput {
    var output_vertex: VertexOutput;
    var x = particle.position.x + input_vertex.offset.x;
    var y = particle.position.y + input_vertex.offset.y;
    output_vertex.clip_position = camera.view_projection * vec4<f32>(x, y, 0.0, 1.0);
    output_vertex.color = particle.color;
    return output_vertex;
}

@fragment
fn main_fragment(output_vertex: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(output_vertex.color, 1.0);
}
