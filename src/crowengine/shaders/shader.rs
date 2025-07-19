extern crate gl;
use nalgebra_glm::{Mat4, Vec3};

pub struct Shader {
    shader_program: u32,
}

impl Shader {
    pub fn new(vertex_shader_source: &str, fragment_shader_source: &str) -> Self {
        let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
        unsafe {
            gl::ShaderSource(
                vertex_shader,
                1,
                &vertex_shader_source.as_bytes().as_ptr().cast(),
                &(vertex_shader_source.len() as i32),
            );
            gl::CompileShader(vertex_shader);
        }

        let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
        unsafe {
            gl::ShaderSource(
                fragment_shader,
                1,
                &fragment_shader_source.as_bytes().as_ptr().cast(),
                &(fragment_shader_source.len() as i32),
            );
            gl::CompileShader(fragment_shader);
        }

        let shader_program = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            gl::DetachShader(shader_program, vertex_shader);
            gl::DetachShader(shader_program, fragment_shader);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        Shader { shader_program }
    }

    pub fn set_color_uniform(&self, color: &Vec3) {
        unsafe {
            let location =
                gl::GetUniformLocation(self.shader_program, b"color\0".as_ptr() as *const _);

            if location != -1 {
                gl::Uniform3f(location, color.x, color.y, color.z);
            } else {
                eprintln!("Uniform 'color' not found!");
            }
        }
    }

    pub fn set_projection_uniform(&self, projection: &Mat4) {
        unsafe {
            let location =
                gl::GetUniformLocation(self.shader_program, b"projection\0".as_ptr() as *const _);

            gl::UniformMatrix4fv(location, 1, gl::FALSE, projection.as_ptr());
        }
    }

    pub fn set_model_uniform(&self, model: &Mat4) {
        unsafe {
            let location =
                gl::GetUniformLocation(self.shader_program, b"model\0".as_ptr() as *const _);

            gl::UniformMatrix4fv(location, 1, gl::FALSE, model.as_ptr());
        }
    }
    pub fn set_view_uniform(&self, view: &Mat4) {
        unsafe {
            let location =
                gl::GetUniformLocation(self.shader_program, b"view\0".as_ptr() as *const _);
            gl::UniformMatrix4fv(location, 1, gl::FALSE, view.as_ptr());
        }
    }

    pub fn set_texture_uniform(&self, name: &str, slot: u32) {
        unsafe {
            let cname = std::ffi::CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.shader_program, cname.as_ptr());
            if location != -1 {
                gl::Uniform1i(location, slot as i32);
            } else {
                eprintln!("Uniform '{}' not found!", name);
            }
        }
    }
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.shader_program);
        }
    }
}
