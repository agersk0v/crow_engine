extern crate gl;

use super::super::shaders::shader::Shader;
use crate::glm;
use nalgebra_glm::{identity, translate, Vec3};

use nalgebra_glm::Mat4;

pub struct Mesh {
    vao: u32,
    indices_count: i32,
}

impl Mesh {
    pub fn new((vertices, indices): (Vec<f32>, Vec<u32>)) -> Self {
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
        }
    }

    pub fn set_color(&self, shader: &Shader, color: &Vec3) {
        shader.set_color_uniform(&color);
    }

    pub fn set_texture(&self, shader: &Shader, name: &str, texture: &u32) {
        shader.set_texture_uniform(name, 0);
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, *texture);
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices_count,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }

    pub fn draw(
        &self,
        shader: &Shader,
        color: &Vec3,
        texture: &u32,
        model_matrix: &Mat4,
        projection: &Mat4,
        view: &Mat4,
    ) {
        shader.use_program();

        self.set_color(&shader, &color);
        shader.set_model_uniform(model_matrix);
        self.set_texture(&shader, "texture1", &texture);
        shader.set_projection_uniform(&projection);
        shader.set_view_uniform(&view);
    }
}
