use crate::geometry::*;

pub fn opposite_point<TC, TT, TO>(mesh: &mut DelaunayMesh2d<TC, TT, TO>, t0i: usize, t1i: usize) -> usize
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

pub fn swap_test_ab<TC, TT, TO>(mesh: &mut DelaunayMesh2d<TC, TT, TO>, ti: usize)
where
    TC: InCircleTest<Point = Vec2d>,
    TT: InTriangleTest,
    TO: TriangleOrientationTest,
{
    let t1 = mesh.triangles[ti].ab;
    let di = opposite_point(mesh, ti, t1);
    let d = &mesh.points[di];
    if mesh.in_circle(ti, d) {
        flip_ab(mesh, ti);
        swap_test_ab(mesh, ti);
        swap_test_ab(mesh, t1);
    }
}


pub fn swap_test_bc<TC, TT, TO>(mesh: &mut DelaunayMesh2d<TC, TT, TO>, ti: usize)
where
    TC: InCircleTest<Point = Vec2d>,
    TT: InTriangleTest,
    TO: TriangleOrientationTest,
{
    let t1 = mesh.triangles[ti].bc;
    let di = opposite_point(mesh, ti, t1);
    let d = &mesh.points[di];
    if mesh.in_circle(ti, d) {
        flip_bc(mesh, ti);
        swap_test_ab(mesh, ti);
        swap_test_ab(mesh, t1);
    }   
}

pub fn swap_test_ca<TC, TT, TO>(mesh: &mut DelaunayMesh2d<TC, TT, TO>, ti: usize)
where
    TC: InCircleTest<Point = Vec2d>,
    TT: InTriangleTest,
    TO: TriangleOrientationTest,
{
    let t1 = mesh.triangles[ti].ca;
    let di = opposite_point(mesh, ti, t1);
    let d = &mesh.points[di];
    if mesh.in_circle(ti, d) {
        flip_ca(mesh, ti);
        swap_test_ab(mesh, ti);
        swap_test_ab(mesh, t1);
    }
}
