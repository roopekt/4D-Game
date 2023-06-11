use super::abstract_material::{Material, ShaderProgramId, ShaderProgramIdContainer, ProgramDescriptor, implement_material_draw, any_uniforms_storage};
use glam::Vec3;

pub const PROGRAM_DESCRIPTORS: [ProgramDescriptor; 3*3] = [
    SingleColorMaterial::NORMAL3D_PROGRAM_DESCRIPTOR, SingleColorMaterial::DEGENERATE3D_PROGRAM_DESCRIPTOR, SingleColorMaterial::DEGENERATE4D_PROGRAM_DESCRIPTOR,
    BlitMaterial::NORMAL3D_PROGRAM_DESCRIPTOR, BlitMaterial::DEGENERATE3D_PROGRAM_DESCRIPTOR, BlitMaterial::DEGENERATE4D_PROGRAM_DESCRIPTOR,
    SingleColorScreenSpaceMaterial::NORMAL3D_PROGRAM_DESCRIPTOR, SingleColorScreenSpaceMaterial::DEGENERATE3D_PROGRAM_DESCRIPTOR, SingleColorScreenSpaceMaterial::DEGENERATE4D_PROGRAM_DESCRIPTOR
];

#[derive(Debug, Copy, Clone)]
pub struct SingleColorMaterial {
    pub albedo_color: Vec3
}
impl Material for SingleColorMaterial {
    const NORMAL3D_PROGRAM_DESCRIPTOR: ProgramDescriptor = ProgramDescriptor::new(
        "3D/default.vert", "3D/single_color.frag");
    const DEGENERATE3D_PROGRAM_DESCRIPTOR: ProgramDescriptor = ProgramDescriptor::new_with_geometry(
        "3D/default_sliced.vert", "3D/single_color.frag", "3D/sliced.geom");
    const DEGENERATE4D_PROGRAM_DESCRIPTOR: ProgramDescriptor = ProgramDescriptor::new_with_geometry(
        "4D/default_sliced.vert", "4D/single_color.frag", "4D/sliced.geom");
    
    const PROGRAM_IDS: ShaderProgramIdContainer = get_program_id_container::<Self>();
    implement_material_draw!(Self::get_uniforms);
}
impl SingleColorMaterial {
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
    const DEGENERATE4D_PROGRAM_DESCRIPTOR: ProgramDescriptor = Self::NORMAL3D_PROGRAM_DESCRIPTOR;
    
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

#[derive(Debug, Copy, Clone)]
pub struct SingleColorScreenSpaceMaterial {
    pub color: Vec3
}
impl Material for SingleColorScreenSpaceMaterial {
    const NORMAL3D_PROGRAM_DESCRIPTOR: ProgramDescriptor = ProgramDescriptor::new(
        "simple/minimal.vert", "simple/unlit_single_color.frag");
    const DEGENERATE3D_PROGRAM_DESCRIPTOR: ProgramDescriptor = Self::NORMAL3D_PROGRAM_DESCRIPTOR;
    const DEGENERATE4D_PROGRAM_DESCRIPTOR: ProgramDescriptor = Self::NORMAL3D_PROGRAM_DESCRIPTOR;
    
    const PROGRAM_IDS: ShaderProgramIdContainer = get_program_id_container::<Self>();
    implement_material_draw!(Self::get_uniforms);
}
impl SingleColorScreenSpaceMaterial {
    pub fn get_uniforms(&self) -> any_uniforms_storage!() {
        glium::uniform! {
            color: self.color.to_array()
        }
    }
}


const fn get_program_id_container<T: Material>() -> ShaderProgramIdContainer {
    ShaderProgramIdContainer {
        normal_3D: get_program_id(T::NORMAL3D_PROGRAM_DESCRIPTOR),
        degenerate_3D: get_program_id(T::DEGENERATE3D_PROGRAM_DESCRIPTOR),
        degenerate_4D: get_program_id(T::DEGENERATE4D_PROGRAM_DESCRIPTOR)
    }
}

//returns an index into PROGRAM_DESCRIPTORS
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