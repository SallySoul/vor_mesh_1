use vor_mesh_1::geometry::*;

fn create_flip_test_mesh() -> Simple2DMesh {
    let mut m = Simple2DMesh::empty();
    let a = m.add_point(vec2![0.0, 0.0]);
    let b = m.add_point(vec2![1.0, 1.0]);
    let c = m.add_point(vec2![0.0, 1.0]);
    let d = m.add_point(vec2![1.0, 0.0]);
    let n1p = m.add_point(vec2![-0.5, 0.5]);
    let n2p = m.add_point(vec2![0.5, -0.5]);
    let n3p = m.add_point(vec2![1.5, 0.5]);
    let n4p = m.add_point(vec2![0.5, 1.5]);
    let t0 = m.add_triangle(a, b, c);
    let t1 = m.add_triangle(d, b, a);
    let n1 = m.add_triangle(n1p, a, c);
    let n2 = m.add_triangle(n2p, d, a);
    let n3 = m.add_triangle(n3p, b, d);
    let n4 = m.add_triangle(n4p, c, b);
    m.set_triangle_neighbors(t0, t1, n4, n1);
    m.set_triangle_neighbors(t1, n3, t0, n2);

    // Add some fake neighbors for other triangles
    let n1_1 = 20;
    let n1_2 = 21;
    m.set_triangle_neighbors(n1, t0, n1_1, n1_2);
    let n2_1 = 22;
    let n2_2 = 23;
    m.set_triangle_neighbors(n2, t1, n2_1, n2_2);
    let n3_1 = 24;
    let n3_2 = 25;
    m.set_triangle_neighbors(n3, t1, n3_1, n3_2);
    let n4_1 = 26;
    let n4_2 = 27;
    m.set_triangle_neighbors(n4, t0, n4_1, n4_2);

    m
}

fn main() {
    let mut m = create_flip_test_mesh();
    let indices: Vec<usize> = (0..m.triangles.len()).collect();
    mesh_2d_to_vtk(&m, &indices, "test_before.vtu");
    flip_ab(&mut m, 0);
    mesh_2d_to_vtk(&m, &indices, "test_after.vtu");
}
