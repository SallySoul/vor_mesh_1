use rand::prelude::*;
use vor_mesh_1::geometry::*;

fn main() {
    let ba = vec2![-100.0, -100.0];
    let bb = vec2![100.0, 0.0];
    let bc = vec2![-100.0, 100.0];
    let mut mesh = Simple2DMesh::bounded(ba, bb, bc);
    let mut ti = 1;
    let mut rng = thread_rng();
    for _ in 0..100 {
        let x: f64 = rng.gen_range(-30.0..30.0);
        let y: f64 = rng.gen_range(-30.0..30.0);
        let pi = mesh.add_point(vec2![x, y]);
        ti = mesh.insert_point(pi, ti).unwrap();
    }

    let indices: Vec<usize> = (1..mesh.triangles.len()).collect();
    mesh_2d_to_vtk(&mesh, &indices, "test.vtu");
    Hdf5Builder::new(&mesh, &"test.hdf5")
        .add_points()
        .add_delaunay_edges()
        .add_vor_diagram()
        .finish();
}
