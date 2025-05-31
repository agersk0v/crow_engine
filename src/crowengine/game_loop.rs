pub fn game_loop() {
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

        cube.draw(
            &shader,
            &xyz(-0.5, 0.0, 1.0),
            &rotate(20.0, 40.0, 40.0),
            &rgb(0.5, 0.0, 0.5),
        );

        window.swap_buffers();
    }
}
