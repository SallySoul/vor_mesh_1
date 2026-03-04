use crate::geometry::*;

/// For a given triangle (t0i),
/// we assert that one of the edges has neighbor t1i,
/// and return the point opposite this edge.
pub fn opposite_point<TC, TT, TO>(
    mesh: &DelaunayMesh2d<TC, TT, TO>,
    t0i: usize,
    t1i: usize,
) -> usize
where
    TC: InCircleTest<Point = Vec2d>,
    TT: InTriangleTest,
    TO: TriangleOrientationTest,
{
    let t1 = mesh.triangles[t1i];
    if t1.ab == t0i {
        t1.c
    } else if t1.bc == t0i {
        t1.a
    } else {
        debug_assert_eq!(t1.ca, t0i);
        t1.b
    }
}

/// Perform in-circle on point opposite edge ab
pub fn swap_test_ab<TC, TT, TO>(mesh: &mut DelaunayMesh2d<TC, TT, TO>, ti: usize)
where
    TC: InCircleTest<Point = Vec2d>,
    TT: InTriangleTest,
    TO: TriangleOrientationTest,
{
    println!("    swap_test_ab, ti: {}", ti);
    let t1 = mesh.triangles[ti].ab;

    if t1 == 0 {
        return;
    }

    let di = opposite_point(mesh, ti, t1);
    let d = &mesh.points[di];
    if mesh.in_circle(ti, d) {
        flip_ab(mesh, ti);
        swap_test_ab(mesh, ti);
        swap_test_ab(mesh, t1);
    }
}

/// Perform in-circle on point opposite edge bc
pub fn swap_test_bc<TC, TT, TO>(mesh: &mut DelaunayMesh2d<TC, TT, TO>, ti: usize)
where
    TC: InCircleTest<Point = Vec2d>,
    TT: InTriangleTest,
    TO: TriangleOrientationTest,
{
    println!("    swap_test_bc, ti: {}", ti);

    let t1 = mesh.triangles[ti].bc;

    if t1 == 0 {
        return;
    }

    let di = opposite_point(mesh, ti, t1);
    let d = &mesh.points[di];
    if mesh.in_circle(ti, d) {
        flip_bc(mesh, ti);
        swap_test_ab(mesh, ti);
        swap_test_ab(mesh, t1);
    }
}

/// Perform in-circle on point opposite edge ca
pub fn swap_test_ca<TC, TT, TO>(mesh: &mut DelaunayMesh2d<TC, TT, TO>, ti: usize)
where
    TC: InCircleTest<Point = Vec2d>,
    TT: InTriangleTest,
    TO: TriangleOrientationTest,
{
    println!("    swap_test_ca, ti: {}", ti);

    let t1 = mesh.triangles[ti].ca;

    if t1 == 0 {
        return;
    }

    let di = opposite_point(mesh, ti, t1);
    let d = &mesh.points[di];
    if mesh.in_circle(ti, d) {
        flip_ca(mesh, ti);
        swap_test_ab(mesh, ti);
        swap_test_ab(mesh, t1);
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn opposite_point_1() {
        let mut m = Simple2DMesh::empty();
        let a = m.add_point(vec2![0.0, 0.0]);
        let b = m.add_point(vec2![1.0, 1.0]);
        let c = m.add_point(vec2![0.0, 1.0]);
        let d = m.add_point(vec2![1.0, 0.0]);
        let t0 = m.add_triangle(a, b, c);
        let t1 = m.add_triangle(d, b, a);
        let n1 = 20;
        let n2 = 21;
        let n3 = 22;
        let n4 = 23;
        m.set_triangle_neighbors(t0, t1, n4, n1);
        m.set_triangle_neighbors(t1, n3, t0, n2);
        assert_eq!(opposite_point(&m, t0, t1), d);
        assert_eq!(opposite_point(&m, t1, t0), c);

        // Rotate t0 to test last case
        let tri_0 = m.mut_triangle(t0);
        tri_0.a = b;
        tri_0.b = c;
        tri_0.c = a;
        tri_0.ab = n4;
        tri_0.bc = n1;
        tri_0.ca = t1;
        assert_eq!(opposite_point(&m, t1, t0), c);
    }
}
