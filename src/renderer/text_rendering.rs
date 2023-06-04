use glium_glyph::glyph_brush::ab_glyph::FontVec;
use crate::global_data::GlobalData;

pub type LoadedFontRef = glium_glyph::glyph_brush::FontId;

pub struct FontContainer {
    pub info_screen: LoadedFontRef
}
impl FontContainer {
    fn load_to_new(loader: &mut FontLoader, global_data: &GlobalData) -> Self {
        Self {
            info_screen: loader.load(&global_data.options.user.info_screen.font_name)
        }
    }
}

pub struct TextRenderer<'a> {
    pub fonts: FontContainer,
    pub brush: glium_glyph::GlyphBrush<'a, FontVec>
}
impl<'a> TextRenderer<'a> {
    pub fn new(display: &glium::Display, global_data: &GlobalData) -> Self {
        let mut brush_builder = glium_glyph::GlyphBrushBuilder::using_fonts(vec![]);
        let mut loader = FontLoader::new(&mut brush_builder);
        let container = FontContainer::load_to_new(&mut loader, global_data);
        let brush = brush_builder.build(display);

        Self {
            fonts: container,
            brush: brush
        }
    }
}

struct FontLoader<'l, 'b> {
    pub system_font_cache: rust_fontconfig::FcFontCache,
    pub brush_builder: &'l mut glium_glyph::GlyphBrushBuilder<'b, FontVec>
}
impl<'l, 'b> FontLoader<'l, 'b> {
    pub fn new(brush_builder: &'l mut glium_glyph::GlyphBrushBuilder<'b, FontVec>) -> Self {
        Self {
            system_font_cache: rust_fontconfig::FcFontCache::build(),
            brush_builder: brush_builder
        }
    }

    pub fn load(&mut self, font_name: &str) -> LoadedFontRef {
        let font_location = self.system_font_cache.query(&rust_fontconfig::FcPattern {
            name: Some(String::from(font_name)),
            .. Default::default()
        }).expect(format!("Can't find font '{}'", font_name).as_str());

        let font_binary = std::fs::read(font_location.path.as_str())
            .expect(format!("Can't open font '{}'", font_name).as_str());

        let font_vec = FontVec::try_from_vec_and_index(font_binary, font_location.font_index as u32)
            .expect(format!("Can't construct FontVec for '{}'", font_name).as_str());

        self.brush_builder.add_font(font_vec)
    }
}