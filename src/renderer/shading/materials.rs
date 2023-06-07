use super::abstract_material::{Material, ShaderProgramId, ShaderProgramIdContainer, ProgramDescriptor, implement_material_draw, any_uniforms_storage};
use glam::Vec3;

pub const PROGRAM_DESCRIPTORS: [ProgramDescriptor; 2*2] = [
    SingleColorMaterial3D::NORMAL3D_PROGRAM_DESCRIPTOR, SingleColorMaterial3D::DEGENERATE3D_PROGRAM_DESCRIPTOR,
    BlitMaterial::NORMAL3D_PROGRAM_DESCRIPTOR, BlitMaterial::DEGENERATE3D_PROGRAM_DESCRIPTOR
];

#[derive(Debug, Copy, Clone)]
pub struct SingleColorMaterial3D {
    pub albedo_color: Vec3
}
impl Material for SingleColorMaterial3D {
    const NORMAL3D_PROGRAM_DESCRIPTOR: ProgramDescriptor = ProgramDescriptor::new(
        "default_3D.vert", "single_color.frag");
    const DEGENERATE3D_PROGRAM_DESCRIPTOR: ProgramDescriptor = ProgramDescriptor::new_with_geometry(
        "default_sliced_3D.vert", "single_color.frag", "sliced_3D.geom");
    
    const PROGRAM_IDS: ShaderProgramIdContainer = get_program_id_container::<Self>();
    implement_material_draw!(Self::get_uniforms);
}
impl SingleColorMaterial3D {
    fn get_uniforms(&self) -> any_uniforms_storage!() {
        glium::uniform! {
            albedo: self.albedo_color.to_array()
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BlitMaterial<'a> {
    pub texture: &'a glium::texture::Texture2d
}
impl Material for BlitMaterial<'_> {
    const NORMAL3D_PROGRAM_DESCRIPTOR: ProgramDescriptor = ProgramDescriptor::new(
        "simple/blit.vert", "simple/blit.frag");
    const DEGENERATE3D_PROGRAM_DESCRIPTOR: ProgramDescriptor = Self::NORMAL3D_PROGRAM_DESCRIPTOR;
    
    const PROGRAM_IDS: ShaderProgramIdContainer = get_program_id_container::<Self>();
    implement_material_draw!(Self::get_uniforms);
}
impl BlitMaterial<'_> {
    pub fn get_uniforms(&self) -> any_uniforms_storage!() {
        glium::uniform! {
            texture_to_blit: self.texture
        }
    }
}


const fn get_program_id_container<T: Material>() -> ShaderProgramIdContainer {
    ShaderProgramIdContainer {
        normal_3D: get_program_id(T::NORMAL3D_PROGRAM_DESCRIPTOR),
        degenerate_3D: get_program_id(T::DEGENERATE3D_PROGRAM_DESCRIPTOR)
    }
}

const fn get_program_id(program_descriptor: ProgramDescriptor) -> ShaderProgramId {
    let mut i: ShaderProgramId = 0;
    while i < PROGRAM_DESCRIPTORS.len() {
        if PROGRAM_DESCRIPTORS[i].is_equal(&program_descriptor) {
            return i;
        }
        i += 1;
    }

    panic!("Program descriptor not in PROGRAM_DESCRIPTORS");
}