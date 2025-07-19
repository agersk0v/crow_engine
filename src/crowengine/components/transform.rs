use nalgebra_glm as glm;

pub struct Transform {
    pub position: glm::Vec3,
    pub rotation: glm::Quat,
    pub scale: glm::Vec3,
}

impl Transform {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: glm::vec3(x, y, z),
            rotation: glm::quat_identity(),
            scale: glm::vec3(1.0, 1.0, 1.0),
        }
    }

    pub fn with_euler_rotation(mut self, x_deg: f32, y_deg: f32, z_deg: f32) -> Self {
        let (x, y, z) = (
            glm::radians(&glm::vec1(x_deg)).x,
            glm::radians(&glm::vec1(y_deg)).x,
            glm::radians(&glm::vec1(z_deg)).x,
        );
        self.rotation = euler_to_quat(x, y, z);
        self
    }

    pub fn view(&self) -> glm::Mat4 {
        self.model_matrix().try_inverse().unwrap_or(glm::identity())
    }

    pub fn with_scale(mut self, x: f32, y: f32, z: f32) -> Self {
        self.scale = glm::vec3(x, y, z);
        self
    }

    pub fn model_matrix(&self) -> glm::Mat4 {
        let translation = glm::translation(&self.position);
        let rotation = glm::quat_to_mat4(&self.rotation);
        let scale = glm::scaling(&self.scale);
        translation * rotation * scale
    }

    pub fn forward(&self) -> glm::Vec3 {
        glm::quat_rotate_vec3(&self.rotation, &glm::vec3(0.0, 0.0, -1.0))
    }

    pub fn up(&self) -> glm::Vec3 {
        glm::quat_rotate_vec3(&self.rotation, &glm::vec3(0.0, 1.0, 0.0))
    }

    pub fn right(&self) -> glm::Vec3 {
        glm::quat_rotate_vec3(&self.rotation, &glm::vec3(1.0, 0.0, 0.0))
    }
}

pub fn euler_to_quat(x: f32, y: f32, z: f32) -> glm::Quat {
    let qx = glm::quat_angle_axis(x, &glm::vec3(1.0, 0.0, 0.0));
    let qy = glm::quat_angle_axis(y, &glm::vec3(0.0, 1.0, 0.0));
    let qz = glm::quat_angle_axis(z, &glm::vec3(0.0, 0.0, 1.0));

    qz * qy * qx
}
