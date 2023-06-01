use std::fs;
use super::abstract_material::{ShaderProgramId, ProgramDescriptor};
use super::materials::PROGRAM_DESCRIPTORS;

pub struct ShaderProgramContainer {
    programs: Vec<glium::Program>
}
impl ShaderProgramContainer {
    pub fn new(display: &glium::Display) -> Self {
        let programs: Vec<glium::Program> = PROGRAM_DESCRIPTORS
            .iter()
            .map(|desc| get_shader_program(&display, desc))
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

fn get_shader_program(display: &glium::Display, descriptor: &ProgramDescriptor) -> glium::Program {
    let path_prefix = "Resources/shaders/";
    let vertex_src = fs::read_to_string(path_prefix.to_owned() + descriptor.vertex_shader_path)
        .expect(format!("Couldn't read shader '{}'", descriptor.vertex_shader_path).as_str());
    let fragment_src = fs::read_to_string(path_prefix.to_owned() + descriptor.fragment_shader_path)
        .expect(format!("Couldn't read shader '{}'", descriptor.fragment_shader_path).as_str());

    match descriptor.geometry_shader_path {
        Some(geometry_path) => {
            let geometry_src = fs::read_to_string(path_prefix.to_owned() + geometry_path)
                .expect(format!("Couldn't read shader '{}'", geometry_path).as_str());
            
            glium::Program::from_source(display, vertex_src.as_str(), fragment_src.as_str(), Some(geometry_src.as_str()))
                .expect(format!("failed to compile: '{}' + '{}' + '{}'", descriptor.vertex_shader_path, descriptor.fragment_shader_path, geometry_path).as_str())
        },
        None => {
            glium::Program::from_source(display, vertex_src.as_str(), fragment_src.as_str(), None)
                .expect(format!("failed to compile: '{}' + '{}'", descriptor.vertex_shader_path, descriptor.fragment_shader_path).as_str())
        }
    }

    
}