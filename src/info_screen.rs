use crate::options::AsVector;
use crate::renderer::text_rendering; 
use crate::global_data::GlobalData;
use crate::game::world::Multiverse;
use glium_glyph::glyph_brush;
use std::fmt::Display;
use glam::Vec2;
use std::f32::consts::TAU;

pub fn render_info_screen(
    target: &mut glium::Frame,
    display: &glium::Display,
    text_renderer: &mut text_rendering::TextRenderer,
    multiverse: &Multiverse,
    global_data: &GlobalData)
{
    let resolution = CustomFormatted(global_data.resolution);
    let capped_FPS = global_data.frame_timings.capped_fps;
    let uncapped_FPS = global_data.frame_timings.uncapped_fps;
    let uncapped_ms_per_frame = global_data.frame_timings.uncapped_milliseconds_per_frame;
    let visual_mode = global_data.visual_mode.to_string();
    let camera_position_3D = CustomFormatted(multiverse.world_3D.player.get_camera_world_position());
    let camera_position_4D = CustomFormatted(multiverse.world_4D.player.get_camera_world_position());
    let look_direction_3D = CustomFormatted(multiverse.world_3D.player.get_pretty_camera_orientation());
    let look_direction_4D = CustomFormatted(multiverse.world_4D.player.get_pretty_camera_orientation());

    let text = format!("\
Resolution: {resolution}
FPS: {capped_FPS:.1}, uncapped 1 / {uncapped_ms_per_frame:.2} ms = {uncapped_FPS:.1}
Mode: {visual_mode}

3D:
Position: {camera_position_3D:.2}
Look direction: {look_direction_3D:.2}

4D:
Position: {camera_position_4D:.2}
Look direction: {look_direction_4D:.2}
");

    let font_size = global_data.options.user.info_screen.font_size;
    let screen_position = global_data.options.user.info_screen.position.as_vector();

    let formatted_text = glyph_brush::Text {
        text: &text,
        scale: font_size.into(),
        font_id: text_renderer.fonts.info_screen,
        ..Default::default()
    };
    let mut section = glyph_brush::Section {
        text: vec![formatted_text],
        ..Default::default()
    };

    //outline
    section.text[0].extra.color = [0.0, 0.0, 0.0, 1.0];
    let outline_size = global_data.options.user.info_screen.relative_outline_size * font_size;
    let offsets = get_points_on_unit_circle(global_data.options.user.info_screen.outline_quality);
    for offset in offsets {
        section.screen_position = (screen_position + outline_size * offset).into();

        text_renderer.brush.queue(section.clone());
    }
    
    //main text
    section.text[0].extra.color = [1.0, 1.0, 1.0, 1.0];
    section.screen_position = screen_position.into();
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
impl Display for CustomFormatted<glam::Vec4> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}, {}, {}, {}",
            format_vector_component(self.0.x, formatter),
            format_vector_component(self.0.y, formatter),
            format_vector_component(self.0.z, formatter),
            format_vector_component(self.0.w, formatter)
        )
    }
}

fn get_points_on_unit_circle(count: usize) -> Vec<Vec2> {
    let rotation = Vec2::from_angle(TAU / count as f32);
    let mut points = Vec::<Vec2>::with_capacity(count);

    for _ in 0..count {
        points.push(match points.last() {
            Some(v) => v.rotate(rotation),
            None => Vec2::X
        });
    }

    points
}
