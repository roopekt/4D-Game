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
                        input_handler.update_key(key, state);
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
                    event::KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Key1), state: ElementState::Pressed, .. } => {
                        global_data.visual_mode = VisualMode::from_int(1);
                    },
                    event::KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Key3), state: ElementState::Pressed, .. } => {
                        global_data.visual_mode = VisualMode::from_int(3);
                    }
                    _ => ()
                }
            },
            event::WindowEvent::Resized(new_size) => {
                global_data.resolution = glam::UVec2::new(new_size.width, new_size.height);
            },
            _ => ()
        },
        event::Event::DeviceEvent { event: device_event, .. } => match device_event {
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