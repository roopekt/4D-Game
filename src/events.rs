pub mod input;

use glium::glutin::event;
use input::InputHandler;
use crate::global_data::GlobalData;

pub fn handle_event(event: event::Event<()>, input_handler: &mut InputHandler, global_data: &mut GlobalData) {
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
            },
            event::WindowEvent::Resized(new_size) => {
                global_data.resolution = glam::UVec2::new(new_size.width, new_size.height);
            },
            _ => ()
        },
        event::Event::DeviceEvent { event: device_event, .. } => match device_event {
            event::DeviceEvent::MouseMotion { delta } => {
                input_handler.add_mouse_delta(delta.0, delta.1);
            },
            _ => ()
        },
        _ => ()
    }
}