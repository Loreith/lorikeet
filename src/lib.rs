extern crate glfw;

use glfw::{Action, Context, Key};

pub fn run() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(300, 300, "Hello this is dog", glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    while !window.should_close() {

        //Handle events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

    }

    println!("Hello, world!");
}

pub fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        },
        _ => {}
    }
}
