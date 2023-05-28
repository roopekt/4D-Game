use super::abstract_material::{Material, ShaderProgramId, ProgramDescriptor, implement_material_draw, any_uniforms_storage};
use glam::Vec3;
use glium::Surface;

pub const PROGRAM_DESCRIPTORS: [ProgramDescriptor; 1] = [SingleColorMaterial3D::PROGRAM_DESCRIPTOR];

pub struct SingleColorMaterial3D {
    pub albedo_color: Vec3
}
impl Material for SingleColorMaterial3D {
    const PROGRAM_DESCRIPTOR: ProgramDescriptor = ProgramDescriptor::new("default_3D.vert", "single_color.frag");
    const PROGRAM_ID: ShaderProgramId = get_program_id::<Self>();
    implement_material_draw!(Self::get_uniforms);
}
impl SingleColorMaterial3D {
    fn get_uniforms(&self) -> any_uniforms_storage!() {
        glium::uniform! {
            albedo: self.albedo_color.to_array()
        }
    }
}

const fn get_program_id<T: Material>() -> ShaderProgramId {
    let mut i: ShaderProgramId = 0;
    while i < PROGRAM_DESCRIPTORS.len() {
        if PROGRAM_DESCRIPTORS[i].is_equal(&T::PROGRAM_DESCRIPTOR) {
            return i;
        }
        i += 1;
    }

    panic!("Program descriptor not in PROGRAM_DESCRIPTORS");
}