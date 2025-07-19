use crate::glm;

pub struct Color {
    pub color: glm::Vec3,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            color: glm::vec3(r, g, b),
        }
    }
}
