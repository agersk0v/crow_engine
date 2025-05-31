extern crate gl;

use super::super::shaders::shader::Shader;

use nalgebra_glm as glm;
use nalgebra_glm::{identity, translate, Vec3};

pub struct Mesh {
    vao: u32,
    indices_count: i32,
    texture: u32,
}

impl Mesh {
    pub fn new(vertices: &[f32], indices: &[u32], texture: u32) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            // VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            // EBO
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            // Vertex attribute

            let stride = (8 * std::mem::size_of::<f32>()) as i32;

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * std::mem::size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(1);

            gl::VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (5 * std::mem::size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(2);
            gl::BindVertexArray(0);
        }

        Mesh {
            vao,
            indices_count: indices.len() as i32,
            texture,
        }
    }
}

impl Mesh {
    pub fn draw(&self, shader: &Shader, position: &Vec3, rotation: &Vec3, color: &Vec3) {
        shader.use_program();
        shader.set_color(color);
        let mut model = identity();
        model = translate(&model, position);
        model = glm::rotate(&model, rotation.x, &glm::vec3(1.0, 0.0, 0.0)); // Pitch
        model = glm::rotate(&model, rotation.y, &glm::vec3(0.0, 1.0, 0.0)); // Yaw
        model = glm::rotate(&model, rotation.z, &glm::vec3(0.0, 0.0, 1.0)); // Roll
        shader.set_model(&model);

        shader.set_texture();
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices_count,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}
