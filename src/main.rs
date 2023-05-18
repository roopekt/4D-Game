pub mod options;
pub mod global_data;
pub mod events;
pub mod game;
pub mod renderer;
extern crate glium;
extern crate rand;

use glium::glutin;
use global_data::GlobalData;
use renderer::Renderer;
use std::time::{Instant, Duration};

fn main() {
    
    let mut global_data = global_data::GlobalData::new();

    let glutin_event_loop = glutin::event_loop::EventLoop::new();
    let display = get_display(&glutin_event_loop, &global_data);
    
    let mut input_handler = events::input::InputHandler::new();
    let mut world = game::world::World::new(&global_data, &display);
    let renderer = Renderer::new(display);

    let mut next_frame_start_time = Instant::now();
    let time_epsilon = Duration::from_micros(100);

    glutin_event_loop.run(move |event, _, control_flow| {
        
        let this_frame_start_time = Instant::now();
        
        events::handle_event(event, &mut input_handler, &mut global_data);
        
        if this_frame_start_time + time_epsilon > next_frame_start_time {
            game::update_game(&mut world, &input_handler, &mut global_data);
            renderer.render_frame(&world, &mut global_data);
            input_handler.reset_deltas();
            
            let single_frame_duration = Duration::from_secs(1).div_f32(global_data.options.user.graphics.max_fps);
            next_frame_start_time = this_frame_start_time + single_frame_duration;
        }
        
        if global_data.close_requested {
            *control_flow = glutin::event_loop::ControlFlow::Exit;
        }
        else {
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_start_time);
        }
    });
}

fn get_display(event_loop: &glutin::event_loop::EventLoop<()>, global_data: &GlobalData) -> glium::Display {
    
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(global_data.resolution.x, global_data.resolution.y));
    let cb = glutin::ContextBuilder::new()
        .with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    {
        let cw = display.gl_window();
        let window = cw.window();
        window.set_cursor_grab(true).unwrap();
        window.set_cursor_visible(false);
    }

    display
}