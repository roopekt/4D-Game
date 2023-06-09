use super::shading::abstract_material::Material;
use super::mesh;
use crate::game::transform;

pub struct RenderableObject3D<M: Material> {
    pub transform: transform::AffineTransform3D,
    pub mesh: mesh::StaticUploadedMesh3D,
    pub material: M
}
pub struct RenderableObject4D<M: Material> {
    pub transform: transform::AffineTransform4D,
    pub mesh: mesh::StaticUploadedMesh4D,
    pub material: M
}
