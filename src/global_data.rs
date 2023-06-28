use crate::options::Options;
use glam::UVec2;

pub struct GlobalData {
    pub close_requested: bool,
    pub resolution: UVec2,
    pub FPS: f32,
    pub uncapped_FPS: f32,//estimate of what the FPS would be without capping
    pub mouse_grabbed: bool,
    pub info_screen_visible: bool,
    pub visual_mode: VisualMode,
    pub polygon_mode: glium::draw_parameters::PolygonMode,//Fill, unless debugging
    pub options: Options
}
impl GlobalData {
    pub fn new() -> Self {
        let options = Options::load();
        GlobalData {
            close_requested: false,
            resolution: UVec2::from_array(options.user.graphics.default_resolution),
            FPS: 0.0,
            uncapped_FPS: 0.0,
            mouse_grabbed: false,
            info_screen_visible: false,
            visual_mode: VisualMode::from_int(options.user.default_mode),
            polygon_mode: glium::draw_parameters::PolygonMode::Fill,
            options: options
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        (self.resolution.x as f32) / (self.resolution.y as f32)
    }

    pub fn reload_options(&mut self) {
        self.options = Options::load();
    }

    pub fn is_4D_active(&self) -> bool {
        self.visual_mode == VisualMode::Degenerate4D
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VisualMode {
    Normal3D,
    Degenerate3D,
    Combined3D,
    Degenerate4D
}
impl VisualMode {
    pub fn from_int(int: u32) -> Self {
        match int {
            1 => Self::Normal3D,
            2 => Self::Combined3D,
            3 => Self::Degenerate3D,
            4 => Self::Degenerate4D,
            _ => panic!("Unknown visual mode {int}")
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}