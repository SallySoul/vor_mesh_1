use vor_mesh_1::geometry::*; 

fn main() {
    let ba = vec2![-100.0, -100.0];
    let bb = vec2![100.0, 0.0];
    let bc = vec2![-100.0, 100.0];
    let mut mesh = Simple2DMesh::bounded(ba, bb, bc);

    let pi = mesh.add_point(vec2![0.0, 0.0]);
    let ti = mesh.insert_point(pi, 0).unwrap();
    
    let pi = mesh.add_point(vec2![-50.0, 0.0]);
    let ti = mesh.insert_point(pi, ti).unwrap();

    let pi = mesh.add_point(vec2![20.0, 20.0]);
    let ti = mesh.insert_point(pi, ti).unwrap();

    let pi = mesh.add_point(vec2![20.0, -20.0]);
    let ti = mesh.insert_point(pi, ti).unwrap();

    let pi = mesh.add_point(vec2![-10.1, -9.3]);
    let ti = mesh.insert_point(pi, ti).unwrap();

    let pi = mesh.add_point(vec2![-11.1, -9.3]);
    let ti = mesh.insert_point(pi, ti).unwrap();

    let pi = mesh.add_point(vec2![-12.8, -16.13]);
    let ti = mesh.insert_point(pi, ti).unwrap();

    let indices: Vec<usize> = (0..mesh.triangles.len()).collect();
    mesh_2d_to_vtk(&mesh, &indices, "test.vtu");
}
