pub mod world;
pub mod transform;
pub mod player;

use world::Multiverse;
use crate::events::input::InputHandler;
use crate::global_data::GlobalData;

pub fn update_game(multiverse: &mut Multiverse, input: &InputHandler, global_data: &mut GlobalData) {

    let delta_time = get_delta_time(multiverse);

    if global_data.is_4D_active() {
        multiverse.world_4D.player.update(delta_time, input, global_data)
    }
    else {
        multiverse.world_3D.player.update(delta_time, input, global_data);
    }
}

pub fn get_delta_time(multiverse: &mut Multiverse) -> f32 {
    let now = std::time::Instant::now();
    let delta_time = (now - multiverse.last_update_time).as_secs_f32();
    multiverse.last_update_time = now;

    delta_time
}