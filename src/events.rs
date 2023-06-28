pub mod input;

use glium::glutin::{event::{self, VirtualKeyCode, ElementState}, window::CursorGrabMode};
use input::InputHandler;
use crate::global_data::{GlobalData, VisualMode};
use std::println;

pub fn handle_event(event: event::Event<()>, input_handler: &mut InputHandler, global_data: &mut GlobalData, display: &glium::Display) {
    match event
    {
        event::Event::WindowEvent { event: win_event, .. } => match win_event
        {
            event::WindowEvent::CloseRequested => {
                global_data.close_requested = true;
            },
            event::WindowEvent::KeyboardInput { input, .. } =>
            {
                match input
                {
                    event::KeyboardInput { virtual_keycode: Some(key), state, .. } => {
                        input_handler.keyboard_update_key(key, state);
                    },
                    _ => ()
                }
                match input
                {
                    event::KeyboardInput { virtual_keycode: Some(VirtualKeyCode::F1), state: ElementState::Pressed, .. } => {
                        global_data.reload_options();
                        println!("Options reloaded");
                    },
                    event::KeyboardInput { virtual_keycode: Some(VirtualKeyCode::F2), state: ElementState::Pressed, .. } => {
                        set_mouse_grab(!global_data.mouse_grabbed, global_data, display);
                    },
                    event::KeyboardInput { virtual_keycode: Some(VirtualKeyCode::F3), state: ElementState::Pressed, .. } => {
                        global_data.info_screen_visible = !global_data.info_screen_visible;
                    },
                    event::KeyboardInput { virtual_keycode: Some(VirtualKeyCode::F4), state: ElementState::Pressed, .. } => {
                        cycle_polygon_mode(global_data);
                    },
                    event::KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Key1), state: ElementState::Pressed, .. } => {
                        global_data.visual_mode = VisualMode::from_int(1);
                    },
                    event::KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Key2), state: ElementState::Pressed, .. } => {
                        global_data.visual_mode = VisualMode::from_int(2);
                    },
                    event::KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Key3), state: ElementState::Pressed, .. } => {
                        global_data.visual_mode = VisualMode::from_int(3);
                    },
                    event::KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Key4), state: ElementState::Pressed, .. } => {
                        global_data.visual_mode = VisualMode::from_int(4);
                    }
                    _ => ()
                }
            },
            event::WindowEvent::MouseInput { button, state, .. } => {
                input_handler.mouse_update_button(button, state);
            },
            event::WindowEvent::Resized(new_size) => {
                global_data.resolution = glam::UVec2::new(new_size.width, new_size.height);
            },
            _ => ()
        },
        event::Event::DeviceEvent { event: device_event, .. } => match device_event {
            // Grabbed here instead of as a window event, because this one is more raw.
            // See the docs for problems with the window event: https://docs.rs/winit/latest/winit/event/enum.WindowEvent.html#variant.CursorMoved
            event::DeviceEvent::MouseMotion { delta } => {
                if global_data.mouse_grabbed {
                    input_handler.add_mouse_delta(delta.0, delta.1);
                }
            },
            _ => ()
        },
        _ => ()
    }
}

pub fn set_mouse_grab(grabbed: bool, global_data: &mut GlobalData, display: &glium::Display) {
    let grab_mode = match grabbed {
        true => CursorGrabMode::Confined,//broken on Mac, iOS, Android and Web
        false => CursorGrabMode::None
    };

    let cw = display.gl_window();
    let window = cw.window();
    window.set_cursor_grab(grab_mode).unwrap();
    window.set_cursor_visible(!grabbed);

    global_data.mouse_grabbed = grabbed;
}

pub fn cycle_polygon_mode(global_data: &mut GlobalData) {
    global_data.polygon_mode = match global_data.polygon_mode {
        glium::PolygonMode::Fill => glium::PolygonMode::Line,
        glium::PolygonMode::Line => glium::PolygonMode::Point,
        glium::PolygonMode::Point => glium::PolygonMode::Fill
    }
}