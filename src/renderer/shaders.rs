use std::fs;

pub struct ShaderProgramContainer {
    pub default: glium::Program
}
impl ShaderProgramContainer {
    pub fn new(display: &glium::Display) -> Self {
        let container = Self {
            default: shader_program(&display, "default_3D.vert", "default.frag")
        };

        display.release_shader_compiler();

        container
    }
}

fn shader_program(display: &glium::Display, vertex_path: &str, fragment_path: &str) -> glium::Program {
    let path_prefix = "Resources/shaders/";
    let vertex_src = fs::read_to_string(path_prefix.to_owned() + vertex_path).unwrap();
    let fragment_src = fs::read_to_string(path_prefix.to_owned() + fragment_path).unwrap();

    glium::Program::from_source(display, vertex_src.as_str(), fragment_src.as_str(), None)
        .expect(format!("failed to compile: '{}' + '{}'", vertex_path, fragment_path).as_str())
}