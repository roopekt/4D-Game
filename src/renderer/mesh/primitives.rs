mod quad;
mod cube;
mod sphere;

pub use quad::*;
pub use cube::*;
pub use sphere::*;

use super::{Mesh3D, Mesh4D, CpuVertex3D, CpuVertex4D, CpuVertexSimple, SimpleMesh};
use glam::Vec3;
use crate::game::transform::Transform3D;

pub fn blit_quad() -> SimpleMesh {
    quad_3D()
        .as_transformed(&Transform3D { scale: 2.0 * Vec3::ONE, ..Default::default() }.into())
        .into()
}

pub fn vertical_line() -> SimpleMesh {
    SimpleMesh {
        vertices: vec![
            CpuVertexSimple { position: Vec3::new(0.0, -1.0, 0.0) },
            CpuVertexSimple { position: Vec3::new(0.0,  1.0, 0.0) }
        ],
        indeces: vec![0, 1],
        topology: glium::index::PrimitiveType::LinesList
    }
}
