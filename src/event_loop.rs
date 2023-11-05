extern crate glfw;

use crate::{application::App, events};
use crate::ui::Imgui;
use glfw::Context;

pub async fn run() {
    // initialize window
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap(); 
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(
        glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core)
    );
    let (mut window, events) = glfw.create_window(
        1000, 
        1000, 
        "mestre dos magos", 
        glfw::WindowMode::Windowed
    ).expect("failed to create glfw window");
    window.make_current();
    glfw.set_swap_interval(glfw::SwapInterval::Sync(0));
    window.set_cursor_pos_polling(true);
    window.set_key_polling(true);
    window.set_scroll_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_mouse_button_polling(true);
    
    //load gl functions
    gl::load_with(|s| window.get_proc_address(s) as * const _);

    let ctx = imgui::Context::create();
    let ui = Imgui::new(ctx, &mut window);
    let mut app = App::new(window, glfw, ui);

    while !app.window_mut().should_close() {
        events::handle_events(&mut app,&events);
        app.update();
        unsafe {
            app.draw();
        }

        app.window_mut().swap_buffers();
        app.glfw_mut().poll_events();

    }
}
