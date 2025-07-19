use crate::crowengine::components::camera::Camera;
use crate::crowengine::components::color::Color;
use crate::crowengine::components::texture::Texture;
use crate::crowengine::components::transform::Transform;
use crate::crowengine::components::use_shader::UseShader;

use nalgebra_glm as glm;

use super::meshes::mesh::Mesh;
use super::world::World;

pub fn render(world: &mut World) {
    for (camera_transform, camera) in world.query::<(&Transform, &Camera)>() {
        let view = camera_transform.view();
        let projection =
            glm::perspective(camera.aspect_ratio, camera.fov_y, camera.near, camera.far);

        for (transform, color, texture, mesh, use_shader) in world.query::<(
            &Transform,
            Option<&Color>,
            &Texture,
            &Mesh,
            Option<&UseShader>,
        )>() {
            mesh.draw(
                UseShader::get_optional_shader(use_shader, world),
                &color.unwrap_or(&Color::new(0., 0., 0.)).color,
                &texture.id,
                &transform.model_matrix(),
                &projection,
                &view,
            );
        }
    }
}
