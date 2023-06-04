use crate::options::AsTuple;
use crate::renderer::text_rendering; 
use crate::global_data::GlobalData;
use crate::game::world::World;
use glium_glyph::glyph_brush;
use std::fmt::Display;

pub fn render_info_screen(
    target: &mut glium::Frame,
    display: &glium::Display,
    text_renderer: &mut text_rendering::TextRenderer,
    world: &World,
    global_data: &GlobalData)
{
    let resolution = CustomFormatted(global_data.resolution);
    let FPS = global_data.FPS;
    let camera_position = CustomFormatted(world.player.get_camera_world_position());
    let look_direction = CustomFormatted(world.player.get_pretty_look_direction());

    let text = format!("\
Resolution: {resolution}
FPS: {FPS:.1}

Position: {camera_position:.2}
Look direction: {look_direction:.2}");

    let formatted_text = glyph_brush::Text {
        text: &text,
        scale: global_data.options.user.info_screen.font_size.into(),
        font_id: text_renderer.fonts.info_screen,
        extra: glyph_brush::Extra { color: [1.0, 1.0, 1.0, 1.0], ..Default::default() }
    };
    let section = glyph_brush::Section {
        screen_position: global_data.options.user.info_screen.position.as_tuple(),
        text: vec![formatted_text],
        ..Default::default()
    };

    text_renderer.brush.queue(section);
    text_renderer.brush.draw_queued(display, target);
}

struct CustomFormatted<V>(V);
fn format_vector_component<T: Display>(value: T, formatter: &std::fmt::Formatter<'_>) -> String {
    match formatter.precision() {
        Some(precision) => format!("{value:.*}", precision),
        None => format!("{value}")
    }
}

impl Display for CustomFormatted<glam::Vec2> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}, {}",
            format_vector_component(self.0.x, formatter),
            format_vector_component(self.0.y, formatter)
        )
    }
}
impl Display for CustomFormatted<glam::UVec2> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}, {}",
            format_vector_component(self.0.x, formatter),
            format_vector_component(self.0.y, formatter)
        )
    }
}
impl Display for CustomFormatted<glam::Vec3> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}, {}, {}",
            format_vector_component(self.0.x, formatter),
            format_vector_component(self.0.y, formatter),
            format_vector_component(self.0.z, formatter)
        )
    }
}
