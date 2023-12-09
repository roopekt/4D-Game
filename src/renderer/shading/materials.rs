use super::abstract_material::{Material, ShaderProgramId, ShaderProgramIdGroup, ProgramDescriptor, ProgramDescriptorGroup, implement_material_draw, any_uniforms_storage};
use glam::Vec3;

const PROGRAM_DESCRIPTOR_GROUP_COUNT: usize = 4;
pub const PROGRAM_DESCRIPTOR_GROUPS: [ProgramDescriptorGroup; PROGRAM_DESCRIPTOR_GROUP_COUNT] = [
    SingleColorMaterial::PROGRAM_DESCRIPTORS,
    ChessboardMaterial::PROGRAM_DESCRIPTORS,
    BlitMaterial::PROGRAM_DESCRIPTORS,
    SingleColorScreenSpaceMaterial::PROGRAM_DESCRIPTORS
];

#[derive(Debug, Copy, Clone)]
pub struct SingleColorMaterial {
    pub albedo_color: Vec3
}
impl Material for SingleColorMaterial {
    const PROGRAM_DESCRIPTORS: ProgramDescriptorGroup = ProgramDescriptorGroup {
        normal_3D: ProgramDescriptor::new(
            "3D/pre_fragment.vert", "3D/single_color.frag"),
        normal_3D_skeleton: ProgramDescriptor::new(
            "3D/pre_fragment.vert", "3D/single_color_skeleton.frag"),
        degenerate_3D: ProgramDescriptor::new_with_geometry(
            "3D/pre_geometry.vert", "3D/single_color.frag", "3D/sliced.geom"),
        degenerate_3D_skeleton: ProgramDescriptor::new_with_geometry(
            "3D/pre_geometry.vert", "3D/single_color_skeleton.frag", "3D/skeleton.geom"),
        degenerate_4D: ProgramDescriptor::new_with_geometry(
            "4D/pre_geometry.vert", "4D/single_color.frag", "4D/sliced.geom"),
        degenerate_4D_skeleton: ProgramDescriptor::new(
            "4D/pre_fragment.vert", "4D/single_color_skeleton.frag")
    };
    
    const PROGRAM_IDS: ShaderProgramIdGroup = get_program_id_container::<Self>();
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
pub struct ChessboardMaterial {
    pub color_A: Vec3,
    pub color_B: Vec3,
    pub square_width: f32
}
impl Material for ChessboardMaterial {
    const PROGRAM_DESCRIPTORS: ProgramDescriptorGroup = ProgramDescriptorGroup {
        normal_3D: ProgramDescriptor::new(
            "3D/pre_fragment.vert", "3D/chessboard.frag"),
        normal_3D_skeleton: ProgramDescriptor::new(
            "3D/pre_fragment.vert", "3D/chessboard_skeleton.frag"),
        degenerate_3D: ProgramDescriptor::new_with_geometry(
            "3D/pre_geometry.vert", "3D/chessboard.frag", "3D/sliced.geom"),
        degenerate_3D_skeleton: ProgramDescriptor::new_with_geometry(
            "3D/pre_geometry.vert", "3D/chessboard_skeleton.frag", "3D/skeleton.geom"),
        degenerate_4D: ProgramDescriptor::new_with_geometry(
            "4D/pre_geometry.vert", "4D/chessboard.frag", "4D/sliced.geom"),
        degenerate_4D_skeleton: ProgramDescriptor::new(
            "4D/pre_fragment.vert", "4D/chessboard_skeleton.frag")
    };
    
    const PROGRAM_IDS: ShaderProgramIdGroup = get_program_id_container::<Self>();
    implement_material_draw!(Self::get_uniforms);
}
impl ChessboardMaterial {
    fn get_uniforms(&self) -> any_uniforms_storage!() {
        glium::uniform! {
            albedo_A: self.color_A.to_array(),
            albedo_B: self.color_B.to_array(),
            square_width: self.square_width
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BlitMaterial<'a> {
    pub texture: &'a glium::texture::Texture2d
}
impl Material for BlitMaterial<'_> {
    const PROGRAM_DESCRIPTORS: ProgramDescriptorGroup = ProgramDescriptorGroup::new_trivial(ProgramDescriptor::new(
        "simple/blit.vert", "simple/blit.frag"));
    const PROGRAM_IDS: ShaderProgramIdGroup = get_program_id_container::<Self>();
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
    const PROGRAM_DESCRIPTORS: ProgramDescriptorGroup = ProgramDescriptorGroup::new_trivial(ProgramDescriptor::new(
        "simple/minimal.vert", "simple/unlit_single_color.frag"));
    const PROGRAM_IDS: ShaderProgramIdGroup = get_program_id_container::<Self>();
    implement_material_draw!(Self::get_uniforms);
}
impl SingleColorScreenSpaceMaterial {
    pub fn get_uniforms(&self) -> any_uniforms_storage!() {
        glium::uniform! {
            color: self.color.to_array()
        }
    }
}


const PROGRAM_DESCRIPTORS_PER_GROUP: usize = 6;
pub const PROGRAM_DESCRIPTORS: [ProgramDescriptor; PROGRAM_DESCRIPTOR_GROUP_COUNT * PROGRAM_DESCRIPTORS_PER_GROUP] = {
    let mut descriptors = [ProgramDescriptor::new("<null>", "<null>"); PROGRAM_DESCRIPTOR_GROUP_COUNT * PROGRAM_DESCRIPTORS_PER_GROUP];
    let mut i = 0;
    while i < PROGRAM_DESCRIPTOR_GROUPS.len() {
        descriptors[i * PROGRAM_DESCRIPTORS_PER_GROUP + 0] = PROGRAM_DESCRIPTOR_GROUPS[i].normal_3D;
        descriptors[i * PROGRAM_DESCRIPTORS_PER_GROUP + 1] = PROGRAM_DESCRIPTOR_GROUPS[i].normal_3D_skeleton;
        descriptors[i * PROGRAM_DESCRIPTORS_PER_GROUP + 2] = PROGRAM_DESCRIPTOR_GROUPS[i].degenerate_3D;
        descriptors[i * PROGRAM_DESCRIPTORS_PER_GROUP + 3] = PROGRAM_DESCRIPTOR_GROUPS[i].degenerate_3D_skeleton;
        descriptors[i * PROGRAM_DESCRIPTORS_PER_GROUP + 4] = PROGRAM_DESCRIPTOR_GROUPS[i].degenerate_4D;
        descriptors[i * PROGRAM_DESCRIPTORS_PER_GROUP + 5] = PROGRAM_DESCRIPTOR_GROUPS[i].degenerate_4D_skeleton;
        i += 1;
    }
    descriptors
};

const fn get_program_id_container<M: Material>() -> ShaderProgramIdGroup {
    ShaderProgramIdGroup {
        normal_3D:              get_program_id(M::PROGRAM_DESCRIPTORS.normal_3D),
        normal_3D_skeleton:     get_program_id(M::PROGRAM_DESCRIPTORS.normal_3D_skeleton),
        degenerate_3D:          get_program_id(M::PROGRAM_DESCRIPTORS.degenerate_3D),
        degenerate_3D_skeleton: get_program_id(M::PROGRAM_DESCRIPTORS.degenerate_3D_skeleton),
        degenerate_4D:          get_program_id(M::PROGRAM_DESCRIPTORS.degenerate_4D),
        degenerate_4D_skeleton: get_program_id(M::PROGRAM_DESCRIPTORS.degenerate_4D_skeleton)
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