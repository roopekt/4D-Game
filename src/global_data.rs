use crate::options::Options;
use glam::UVec2;

pub struct GlobalData {
    pub close_requested: bool,
    pub resolution: UVec2,
    pub FPS: f32,
    pub options: Options,
    pub mouse_grabbed: bool,
    pub info_screen_visible: bool
}
impl GlobalData {
    pub fn new() -> Self {
        let options = Options::load();
        GlobalData {
            close_requested: false,
            resolution: UVec2::from_array(options.user.graphics.default_resolution),
            FPS: 0.0,
            options: options,
            mouse_grabbed: false,
            info_screen_visible: false
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        (self.resolution.x as f32) / (self.resolution.y as f32)
    }

    pub fn reload_options(&mut self) {
        self.options = Options::load();
    }
}