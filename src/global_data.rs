use crate::options::Options;
use glam::UVec2;

pub struct GlobalData {
    pub close_requested: bool,
    pub resolution: UVec2,
    pub options: Options
}
impl GlobalData {
    pub fn new() -> Self {
        let options = Options::load();
        GlobalData {
            close_requested: false,
            resolution: UVec2::from_array(options.user.graphics.default_resolution),
            options: options
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        (self.resolution.x as f32) / (self.resolution.y as f32)
    }
}