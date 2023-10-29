pub struct Camera {
    pub eye: glm::Vec3,
    pub target: glm::Vec3,
    pub up: glm::Vec3,
    pub aspect: f32,
    pub fov: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Camera {
    pub fn generate_camera_matrix(&self) -> glm::Mat4 {
        #[rustfmt::skip]
        let opengl_to_wgpu = glm::mat4(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.5, 0.5,
            0.0, 0.0, 0.0, 1.0,
        );

        let view = glm::look_at_rh(&self.eye, &self.target, &self.up);
        let projection = glm::perspective(self.aspect, self.fov, self.z_near, self.z_far);
        return opengl_to_wgpu * projection * view;
    }
}

pub struct CameraController {
    pub speed: f32,
    pub is_forward_pressed: bool,
    pub is_backward_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }
}
