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
use crate::crowengine::meshes::mesh::Mesh;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Entity {
    pub name: String,
    pub mesh: Mesh,
    pub position: glm::Vec3,
    pub rotation: glm::Vec3,
    pub color: glm::Vec3,
}

pub struct Entities {
    pub list: Vec<Entity>,
}

impl Entities {
    pub fn new() -> Self {
        Entities { list: vec![] }
    }

    pub fn render(&self, shader: &Shader) {
        for entity in &self.list {
            entity
                .mesh
                .draw(&shader, &entity.position, &entity.rotation, &entity.color);
        }
    }

    pub fn add(&mut self, entity: Entity) {
        self.list.push(entity);
    }
}

pub struct Commands {
    pub entities: Rc<RefCell<Entities>>,
}

impl Commands {
    pub fn new(entities: Rc<RefCell<Entities>>) -> Self {
        Commands { entities }
    }

    pub fn spawn(
        &self,
        mesh: Mesh,
        name: &str,
        position: glm::Vec3,
        rotation: glm::Vec3,
        color: glm::Vec3,
    ) {
        let entity = Entity {
            name: name.to_string(),
            mesh,
            position,
            rotation,
            color,
        };

        self.entities.borrow_mut().add(entity);
    }
}

fn main() {
    let (mut glfw, mut window, events) = create_window();

    let (width, height) = window.get_framebuffer_size();

    let aspect = width as f32 / height as f32;

    let fov = glm::radians(&glm::vec1(45.0)).x;
    let projection = glm::perspective(aspect, fov, 0.1, 100.0);

    // camera
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

    let entities = Rc::new(RefCell::new(Entities::new()));
    let commands = Commands::new(Rc::clone(&entities));

    let cube = Mesh::new(&vertices, &indices, texture);

    commands.spawn(
        cube,
        "Cube",
        xyz(-0.5, 0.0, 1.0),
        rotate(20.0, 40.0, 40.0),
        rgb(0.5, 0.0, 0.5),
    );

    let cube = Mesh::new(&vertices, &indices, texture);

    commands.spawn(
        cube,
        "Cube",
        xyz(0.5, 0.0, 1.0),
        rotate(20.0, 40.0, 40.0),
        rgb(0.5, 0.0, 0.5),
    );

    while !window.should_close() {
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            process_input(&mut window, event);
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);

            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        entities.borrow().render(&shader);

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
