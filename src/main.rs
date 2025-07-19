extern crate gl;
extern crate glfw;
use glfw::Context;

use nalgebra_glm as glm;

extern crate image;

mod window;
use window::create_window::create_window;

mod crowengine;
use crate::crowengine::shaders::{
    fragment_shader::FRAGMENT_SHADER_SOURCE, shader::Shader, vertex_shader::VERTEX_SHADER_SOURCE,
};

use crate::crowengine::loaders::load_texture::load_texture;
use crate::crowengine::meshes::cube::Cube;
use crate::crowengine::meshes::mesh::Mesh;

use crate::crowengine::commands::Commands;

use crate::crowengine::world::World;

use crate::crowengine::components::color::Color;
use crate::crowengine::components::texture::Texture;
use crate::crowengine::components::transform::Transform;

use crate::crowengine::components::use_shader::UseShader;

use crate::crowengine::components::camera::Camera;

use crate::crowengine::render::render;

fn main() {
    let (mut glfw, mut window, events) = create_window();

    let (width, height) = window.get_framebuffer_size();

    let texture = load_texture("src/assets/brick_texture.jpg");

    let mut world = World::new(Shader::new(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE));

    let mut commands = Commands::new(&mut world);

    commands.spawn((
        Camera {
            fov_y: glm::radians(&glm::vec1(45.0)).x,
            near: 0.1,
            far: 100.0,
            aspect_ratio: width as f32 / height as f32,
        },
        Transform::from_xyz(0.0, 0.0, 3.0),
    ));

    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0).with_euler_rotation(20., 20., 20.),
        Mesh::new(Cube::new(0.5, 0.5, 0.5)),
        Texture::new(texture),
        UseShader::new("default".to_string()),
        Color::new(1., 1., 1.),
    ));

    commands.spawn((
        Transform::from_xyz(0.5, 0.5, 0.0).with_euler_rotation(20., 20., 20.),
        Mesh::new(Cube::new(0.5, 0.5, 0.5)),
        Texture::new(texture),
        Color::new(1., 0., 0.),
    ));

    commands.spawn((
        Transform::from_xyz(-0.5, 0.5, 0.0).with_euler_rotation(20., 20., 20.),
        Mesh::new(Cube::new(0.5, 0.5, 0.5)),
        Texture::new(texture),
    ));

    while !window.should_close() {
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
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
                _ => {}
            }
        }

        process_input(&mut window, &mut world);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);

            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        render(&mut world);

        window.swap_buffers();
    }
}

fn process_input(window: &mut glfw::Window, world: &mut World) {
    use glfw::Action;
    use glfw::Key;

    if window.get_key(Key::W) == Action::Press {
        for (_, transform) in world.query_mut::<Transform>() {
            transform.position.z += 0.1;
        }
    }

    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
    }

    if window.get_key(Key::G) == Action::Press {
        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        }
    }

    if window.get_key(Key::F) == Action::Press {
        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        }
    }
}
