#![allow(non_snake_case)]// will otherwise complain about 3D and 4D not being in snake case

pub mod options;
pub mod global_data;
pub mod events;
pub mod game;
pub mod renderer;
pub mod info_screen;
pub mod errors;

use glium::glutin;
use global_data::GlobalData;
use renderer::Renderer;
use std::time::Instant;
use spin_sleep::LoopHelper;

fn main() {
    
    let mut global_data = global_data::GlobalData::new();

    let glutin_event_loop = glutin::event_loop::EventLoop::new();
    let display = get_display(&glutin_event_loop, &global_data);
    
    let mut input_handler = events::input::InputHandler::new();
    let mut world = game::world::World3D::new(&global_data, &display);
    let mut renderer = Renderer::new(&display, &global_data);

    events::set_mouse_grab(true, &mut global_data, &display);

    let mut frame_start_instant = Instant::now();

    let mut max_FPS = global_data.options.user.graphics.max_fps;
    let mut clock = LoopHelper::builder()
        .report_interval_s(0.5)
        .build_with_target_rate(max_FPS);

    glutin_event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::MainEventsCleared =>
            {
                //the end measurement is here so that event handling gets measured as well
                let frame_end_instant = Instant::now();
                global_data.uncapped_FPS = 1.0 / (frame_end_instant - frame_start_instant).as_secs_f32();

                clock.loop_sleep();

                frame_start_instant = Instant::now();
                clock.loop_start();
                let correct_max_FPS = global_data.options.user.graphics.max_fps;
                if max_FPS != correct_max_FPS {
                    max_FPS = correct_max_FPS;
                    clock.set_target_rate(max_FPS);
                }
                if let Some(FPS) = clock.report_rate() {
                    global_data.FPS = FPS as f32;
                }

                game::update_game(&mut world, &input_handler, &mut global_data);
                renderer.render_frame(&display, &world, &mut global_data);
                input_handler.reset_deltas();
            },
            other => {
                events::handle_event(other, &mut input_handler, &mut global_data, &display);
            }
        }

        if global_data.close_requested {
            *control_flow = glutin::event_loop::ControlFlow::Exit;
        }
    });
}

fn get_display(event_loop: &glutin::event_loop::EventLoop<()>, global_data: &GlobalData) -> glium::Display {
    
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(global_data.resolution.x, global_data.resolution.y));
    let cb = glutin::ContextBuilder::new()
        .with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    display
}