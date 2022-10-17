mod delaunay_mesh_2d;
mod delaunay_mesh_2d_vtk;
mod edge_flip_2d;
mod triangle;

mod geom_types;
mod in_circle;
mod in_circle_simple_det;
mod in_triangle;
mod in_triangle_simple;
mod orient_triangle;
mod orient_triangle_simple;

pub use delaunay_mesh_2d::*;
pub use delaunay_mesh_2d_vtk::*;
pub use edge_flip_2d::*;
pub use triangle::*;
pub use geom_types::*;
pub use in_circle::*;
pub use in_circle_simple_det::*;
pub use in_triangle::*;
pub use in_triangle_simple::*;
pub use orient_triangle::*;
pub use orient_triangle_simple::*;

pub type Simple2DMesh = DelaunayMesh2d<SimpleDetInCircle, InTriangleSimple, TriangleOrientationSimple>;
