use crate::application::App;
use glfw::{Key, Action, WindowEvent};
use std::sync::mpsc::Receiver;

pub fn handle_events(app: &mut App, events: &Receiver<(f64, WindowEvent)>) {
    for (_, event) in glfw::flush_messages(&events) {
        match event {
            glfw::WindowEvent::CursorPos(x, y) => {
                let (x, y) = (x as f32, y as f32);
                app.mouse(x, y);
                app.ui.on_mouse_move(x, y);
            },
            glfw::WindowEvent::MouseButton(button, action, _) => {
                app.ui.on_mouse_click(button, action);
            }
            glfw::WindowEvent::Scroll(x, y) => {
                app.ui.on_mouse_scroll(x as f32, y as f32);
            }
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe {
                    gl::Viewport(0, 0, width, height);
                }
            },
            glfw::WindowEvent::Key(Key::LeftAlt, _, Action::Press, _) => {
                if app.window_mut().get_cursor_mode() == glfw::CursorMode::Disabled {
                    app.window_mut().set_cursor_mode(glfw::CursorMode::Normal);
                } else {
                    app.window_mut().set_cursor_mode(glfw::CursorMode::Disabled);
                }
            }
            glfw::WindowEvent::Key(Key::F2, _, Action::Press, _) => {
                unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }
            }
            glfw::WindowEvent::Key(Key::F3, _, Action::Press, _) => {
                unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL); }
            }
            glfw::WindowEvent::Key(Key::F1, _, Action::Press, _) => {
                unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::POINT); }
            }
            _ => {},
        }
    }
}
