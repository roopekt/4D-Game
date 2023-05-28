use std::fs;
use super::abstract_material::ShaderProgramId;
use super::materials::PROGRAM_DESCRIPTORS;

pub struct ShaderProgramContainer {
    programs: Vec<glium::Program>
}
impl ShaderProgramContainer {
    pub fn new(display: &glium::Display) -> Self {
        let programs: Vec<glium::Program> = PROGRAM_DESCRIPTORS
            .iter()
            .map(|desc| get_shader_program(&display, desc.vertex_shader_path, desc.fragment_shader_path))
            .collect();

        display.release_shader_compiler();

        Self {
            programs: programs
        }
    }

    pub fn get_program(&self, id: ShaderProgramId) -> &glium::Program {
        &self.programs[id]
    }
}

fn get_shader_program(display: &glium::Display, vertex_path: &str, fragment_path: &str) -> glium::Program {
    let path_prefix = "Resources/shaders/";
    let vertex_src = fs::read_to_string(path_prefix.to_owned() + vertex_path).unwrap();
    let fragment_src = fs::read_to_string(path_prefix.to_owned() + fragment_path).unwrap();

    glium::Program::from_source(display, vertex_src.as_str(), fragment_src.as_str(), None)
        .expect(format!("failed to compile: '{}' + '{}'", vertex_path, fragment_path).as_str())
}