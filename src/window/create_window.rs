extern crate gl;
extern crate glfw;
use glfw::Context;

const WIDTH: u32 = 480;
const HEIGHT: u32 = 320;
const TITLE: &str = "Crow Engine";

pub fn create_window() -> (
    glfw::Glfw,
    glfw::PWindow,
    glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
) {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw::init(fail_on_errors!()).expect("Failed to initialize GLFW");
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::Resizable(false));

    let (mut window, events) = glfw
        .with_primary_monitor(|glfw, m| {
            if let Some(monitor) = m {
                if let Some(vidmode) = monitor.get_video_mode() {
                    glfw.create_window(
                        vidmode.width,
                        vidmode.height,
                        TITLE,
                        glfw::WindowMode::FullScreen(&monitor),
                    )
                } else {
                    panic!("Failed to get video mode for monitor.");
                }
            } else {
                glfw.create_window(WIDTH, HEIGHT, TITLE, glfw::WindowMode::Windowed)
            }
        })
        .expect("Failed to create GLFW window");
    let (screen_width, screen_height) = window.get_framebuffer_size();

    window.make_current();
    window.set_key_polling(true);
    gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

    unsafe {
        gl::Viewport(0, 0, screen_width, screen_height);
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    };

    (glfw, window, events)
}
