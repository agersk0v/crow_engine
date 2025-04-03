extern crate gl;
extern crate glfw;
use nalgebra_glm as glm;

extern crate image;

use image::GenericImageView;
use std::path::Path;

use glfw::Context;

mod setup;
use setup::setup::setup;

mod shaders;
use shaders::{
    fragment_shader::FRAGMENT_SHADER_SOURCE, shader::Shader, vertex_shader::VERTEX_SHADER_SOURCE,
};

mod meshes;
use meshes::mesh::Mesh;

fn main() {
    let (mut glfw, mut window, events) = setup();

    let (width, height) = window.get_framebuffer_size();
    let aspect = width as f32 / height as f32;

    let fov = glm::radians(&glm::vec1(45.0)).x;
    let projection = glm::perspective(aspect, fov, 0.1, 100.0);

    let eye = glm::vec3(0.0, 0.0, 3.0); // camera position
    let center = glm::vec3(0.0, 0.0, 0.0); // look at origin
    let up = glm::vec3(0.0, 1.0, 0.0); // camera up

    let view = glm::look_at(&eye, &center, &up);

    let texture = load_texture("src/assets/brick_texture.jpg");

    let shader = Shader::new(
        VERTEX_SHADER_SOURCE,
        FRAGMENT_SHADER_SOURCE,
        projection,
        view,
    );

    let vertices: Vec<f32> = vec![
        // Front face
        -0.25, -0.25, 0.25, 0.0, 0.0, 1.0, 1.0, 1.0, 0.25, -0.25, 0.25, 1.0, 0.0, 1.0, 1.0, 1.0,
        0.25, 0.25, 0.25, 1.0, 1.0, 1.0, 1.0, 1.0, -0.25, 0.25, 0.25, 0.0, 1.0, 1.0, 1.0, 1.0,
        // Back face
        -0.25, -0.25, -0.25, 1.0, 0.0, 1.0, 1.0, 1.0, 0.25, -0.25, -0.25, 0.0, 0.0, 1.0, 1.0, 1.0,
        0.25, 0.25, -0.25, 0.0, 1.0, 1.0, 1.0, 1.0, -0.25, 0.25, -0.25, 1.0, 1.0, 1.0, 1.0, 1.0,
        // Left face
        -0.25, -0.25, -0.25, 0.0, 0.0, 1.0, 1.0, 1.0, -0.25, -0.25, 0.25, 1.0, 0.0, 1.0, 1.0, 1.0,
        -0.25, 0.25, 0.25, 1.0, 1.0, 1.0, 1.0, 1.0, -0.25, 0.25, -0.25, 0.0, 1.0, 1.0, 1.0, 1.0,
        // Right face
        0.25, -0.25, 0.25, 0.0, 0.0, 1.0, 1.0, 1.0, 0.25, -0.25, -0.25, 1.0, 0.0, 1.0, 1.0, 1.0,
        0.25, 0.25, -0.25, 1.0, 1.0, 1.0, 1.0, 1.0, 0.25, 0.25, 0.25, 0.0, 1.0, 1.0, 1.0, 1.0,
        // Top face
        -0.25, 0.25, 0.25, 0.0, 0.0, 1.0, 1.0, 1.0, 0.25, 0.25, 0.25, 1.0, 0.0, 1.0, 1.0, 1.0, 0.25,
        0.25, -0.25, 1.0, 1.0, 1.0, 1.0, 1.0, -0.25, 0.25, -0.25, 0.0, 1.0, 1.0, 1.0, 1.0,
        // Bottom face
        -0.25, -0.25, -0.25, 0.0, 0.0, 1.0, 1.0, 1.0, 0.25, -0.25, -0.25, 1.0, 0.0, 1.0, 1.0, 1.0,
        0.25, -0.25, 0.25, 1.0, 1.0, 1.0, 1.0, 1.0, -0.25, -0.25, 0.25, 0.0, 1.0, 1.0, 1.0, 1.0,
    ];

    let indices: Vec<u32> = vec![
        // Front face
        0, 1, 2, 2, 3, 0, // Back face
        4, 5, 6, 6, 7, 4, // Left face
        8, 9, 10, 10, 11, 8, // Right face
        12, 13, 14, 14, 15, 12, // Top face
        16, 17, 18, 18, 19, 16, // Bottom face
        20, 21, 22, 22, 23, 20,
    ];

    let cube = Mesh::new(&vertices, &indices, texture);

    while !window.should_close() {
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            process_input(&mut window, event);
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let time = glfw.get_time() as f32;

        cube.draw(
            &shader,
            &xyz(0.5, 0.0, 0.0),
            &rotate(40.0 * time, 40.0 * time, 40.0 * time),
            &rgb(1.0, 0.0, 0.0),
        );
        window.swap_buffers();
    }
}

fn rotate(x: f32, y: f32, z: f32) -> glm::Vec3 {
    glm::vec3(
        glm::radians(&glm::vec1(x)).x,
        glm::radians(&glm::vec1(y)).x,
        glm::radians(&glm::vec1(z)).x,
    )
}

fn xyz(x: f32, y: f32, z: f32) -> glm::Vec3 {
    glm::vec3(x, y, z)
}

fn rgb(r: f32, g: f32, b: f32) -> glm::Vec3 {
    glm::vec3(r, g, b)
}

fn process_input(window: &mut glfw::Window, event: glfw::WindowEvent) {
    use glfw::Action;
    use glfw::Key;
    use glfw::WindowEvent as Event;

    match event {
        Event::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        Event::FramebufferSize(width, height) => unsafe {
            gl::Viewport(0, 0, width, height);
        },
        Event::Key(Key::W, _, Action::Press, _) => unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        },
        Event::Key(Key::F, _, Action::Press, _) => unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        },

        _ => {}
    }
}

fn load_texture(path: &str) -> u32 {
    let img = image::open(&Path::new(path)).expect("Failed to load texture");
    let img = img.flipv();
    let data = img.to_rgba8();

    let (width, height) = img.dimensions();
    let mut texture = 0;

    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const _,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        // Texture parameters (wrap, filter)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    }

    texture
}
