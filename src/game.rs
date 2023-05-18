pub mod world;
pub mod transform;
pub mod player;

use world::World;
use crate::events::input::InputHandler;
use crate::global_data::GlobalData;

pub fn update_game(world: &mut World, input: &InputHandler, global_data: &mut GlobalData) {

    let delta_time = delta_time(world);

    world.player.update(delta_time, input, global_data);
}

pub fn delta_time(world: &mut World) -> f32 {
    let now = std::time::Instant::now();
    let delta_time = (now - world.last_update_time).as_secs_f32();
    world.last_update_time = now;

    delta_time
}