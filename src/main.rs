#![allow(non_snake_case)]// will otherwise complain about 3D and 4D not being in snake case

pub mod options;
pub mod global_data;
pub mod events;
pub mod game;
pub mod renderer;
pub mod info_screen;
pub mod errors;
pub mod clock;
pub mod combinations;

use glium::glutin;

fn main() {
    assert_request_for_best_gpu_made_windows();

    let mut global_data = global_data::GlobalData::new();

    let glutin_event_loop = glutin::event_loop::EventLoop::new();
    let display = get_display(&glutin_event_loop, &global_data);
    
    let mut input_handler = events::input::InputHandler::new();
    let mut renderer = renderer::Renderer::new(&display, &global_data);
    let mut multiverse = game::world::Multiverse::new(&global_data, &display);
    let mut clock = clock::MainLoopClock::new();

    events::set_mouse_grab(true, &mut global_data, &display);

    glutin_event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::MainEventsCleared =>
            {
                game::update_game(&mut multiverse, &input_handler, &mut global_data);
                renderer.render_frame(&display, &multiverse, &mut global_data);
                input_handler.reset_deltas();

                let is_end_of_measurement_interval = clock.tick(global_data.options.user.graphics.max_fps);
                global_data.frame_timings = clock.average_frame_timgings();
                if is_end_of_measurement_interval {
                    display.gl_window().window().set_title(&format!("4D game | {:.2} ms/f", global_data.frame_timings.uncapped_milliseconds_per_frame));
                }
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

fn get_display(event_loop: &glutin::event_loop::EventLoop<()>, global_data: &global_data::GlobalData) -> glium::Display {
    
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(global_data.resolution.x, global_data.resolution.y))
        .with_title("4D game");
    let cb = glutin::ContextBuilder::new()
        .with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    display
}

//download more GPU
#[cfg(windows)]
extern "C" {
    static NvOptimusEnablement: u32;
    static AmdPowerXpressRequestHighPerformance: i32;
}
fn assert_request_for_best_gpu_made_windows() {
    #[cfg(windows)]
    unsafe {
        assert_eq!(NvOptimusEnablement, 1);
        assert_eq!(AmdPowerXpressRequestHighPerformance, 1);
    }
}