use super::shading::abstract_material::Material;
use super::mesh::StaticUploadedMesh;
use crate::game::transform::AffineTransform3D;

pub struct RenderableObject<M: Material> {
    pub transform: AffineTransform3D,
    pub mesh: StaticUploadedMesh,
    pub material: M
}
