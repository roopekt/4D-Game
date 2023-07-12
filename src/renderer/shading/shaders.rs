use std::fs;
use super::abstract_material::{ShaderProgramId, ProgramDescriptor};
use super::materials::PROGRAM_DESCRIPTORS;
use itertools::Itertools;

//regex, that is only compiled once
macro_rules! regex {
    ($regex_literal:literal $(,)?) => {{
        static REGEX_CELL: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        REGEX_CELL.get_or_init(|| regex::Regex::new($regex_literal).unwrap())
    }};
}

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
    let vertex_src = read_shader_source(descriptor.vertex_shader_path);
    let fragment_src = read_shader_source(descriptor.fragment_shader_path);
    let geometry_src = descriptor.geometry_shader_path.map(|path| read_shader_source(path));

    let compilation_result = glium::Program::from_source(
        display,
        &vertex_src,
        &fragment_src,
        geometry_src.as_deref()
    );
    compilation_result.unwrap_or_else(|error| {
        on_program_creation_error(
            error,
            descriptor.vertex_shader_path,
            &vertex_src,
            descriptor.fragment_shader_path,
            &fragment_src,
            descriptor.geometry_shader_path,
            geometry_src.as_deref()
        )
    })
}

fn read_shader_source(path: &str) -> String {
    let source = fs::read_to_string("Resources/shaders/".to_owned() + path)
        .expect(&format!("Couldn't read shader '{}'", path));

    let source = source.replace("\r\n", "\n");

    let include_pattern = regex!("//@include (.*)");
    let source = include_pattern.replace_all(&source, |captures: &regex::Captures| {
        let path = &captures[1];
        let extra_source = read_shader_source(path);
        format!("//include start {path}\n{extra_source}\n//include end {path}")
    });

    source.to_string()
}

fn on_program_creation_error(
    error: glium::ProgramCreationError,
    vertex_path: &str,
    vertex_source: &str,
    fragment_path: &str,
    fragment_source: &str,
    geometry_path: Option<&str>,
    geometry_source: Option<&str>) -> !
{
    let main_error_message = match error {
        glium::ProgramCreationError::CompilationError(error, stage) => format!("{stage:?} CompilationError: {error}"),
        glium::ProgramCreationError::LinkingError(error) => format!("LinkingError: {error}"),
        other_variant => format!("{other_variant:?}")
    };

    const GEOMETRY_STAGE_NULL: &str = "<no geometry stage>";
    let geometry_path = geometry_path.unwrap_or(GEOMETRY_STAGE_NULL);

    let vertex_source = add_line_numbers(vertex_source);
    let fragment_source = add_line_numbers(fragment_source);
    let geometry_source = geometry_source.map_or(GEOMETRY_STAGE_NULL.to_owned(), |source| add_line_numbers(source));

    panic!("\
failed to compile: '{vertex_path}' + '{fragment_path}' + '{geometry_path}'
{main_error_message}

===VERTEX===
{vertex_source}
==FRAGMENT==
{fragment_source}
==GEOMETRY==
{geometry_source}
============
");
}

fn add_line_numbers(source_code: &str) -> String {
    source_code
        .lines()
        .enumerate()
        .map(|(i, line)| format!("{i:>3} {line}"))
        .join("\n")
}