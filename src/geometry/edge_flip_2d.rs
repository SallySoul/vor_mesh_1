use crate::geometry::*;

pub fn flip_ab<TC, TT, TO>(mesh: &mut DelaunayMesh2d<TC, TT, TO>, ti: usize)
where
    TC: InCircleTest<Point = Vec2d>,
    TT: InTriangleTest,
    TO: TriangleOrientationTest,
{
    let t = mesh.triangles[ti];
    flip(mesh, t, ti);
}

pub fn flip_bc<TC, TT, TO>(mesh: &mut DelaunayMesh2d<TC, TT, TO>, ti: usize)
where
    TC: InCircleTest<Point = Vec2d>,
    TT: InTriangleTest,
    TO: TriangleOrientationTest,
{
    let t_ref = mesh.triangles[ti];
    let t = Triangle {
        a: t_ref.b,
        b: t_ref.c,
        c: t_ref.a,
        ab: t_ref.bc,
        bc: t_ref.ca,
        ca: t_ref.ab,
    };
    debug_assert!(t.orientation(mesh) == TriangleOrientation::Positive);
    flip(mesh, t, ti);
}

pub fn flip_ca<TC, TT, TO>(mesh: &mut DelaunayMesh2d<TC, TT, TO>, ti: usize)
where
    TC: InCircleTest<Point = Vec2d>,
    TT: InTriangleTest,
    TO: TriangleOrientationTest,
{
    let t_ref = mesh.triangles[ti];
    let t = Triangle {
        a: t_ref.c,
        b: t_ref.a,
        c: t_ref.b,
        ab: t_ref.ca,
        bc: t_ref.ab,
        ca: t_ref.bc,
    };
    flip(mesh, t, ti);
}

/// For a given triangle (ni),
/// we assert that one of the edges has neighbor t0
/// and we want it to be t1
pub fn swap_neighbor<TC, TT, TO>(
    mesh: &mut DelaunayMesh2d<TC, TT, TO>,
    ni: usize,
    t0: usize,
    t1: usize,
) where
    TC: InCircleTest<Point = Vec2d>,
    TT: InTriangleTest,
    TO: TriangleOrientationTest,
{
    let n = &mut mesh.triangles[ni];
    if n.ab == t0 {
        n.ab = t1;
    } else if n.bc == t0 {
        n.bc = t1;
    } else {
        debug_assert!(n.ca == t0);
        n.ca = t1;
    }
}

/// We implement one flip operation
/// However, we pass in a triangle that has been oriented such that
/// Its always an ab flip
/// ```text
///  C    n4    B         C    n4    B
///   ┌────────┐           ┌────────┐
///   │       X│           │X       │
///   │ t0   X │           │ X  t1  │
///   │     X  │           │  X     │
/// n1│    X   │n3 ────► n1│   X    │n3
///   │   X    │           │    X   │
///   │  X     │           │     X  │
///   │ X   t1 │           │ t0   X │
///   │X       │           │       X│
///   └────────┘           └────────┘
///  A    n2    D         A    n2    D
///  ```
///  This means that the output of this function will always be the same regardless
///  of the ordering of the input triangles.
///  The output will always have t0 as `ADC` and t1 `DBC`
fn flip<TC, TT, TO>(mesh: &mut DelaunayMesh2d<TC, TT, TO>, t0: Triangle, t0i: usize)
where
    TC: InCircleTest<Point = Vec2d>,
    TT: InTriangleTest,
    TO: TriangleOrientationTest,
{
    let a;
    let b;
    let c;
    let d;
    let n1;
    let n2;
    let n3;
    let n4;
    let t1i;
    {
        t1i = t0.ab;
        let t1 = &mesh.triangles[t1i];
        debug_assert!(t1.orientation(mesh) == TriangleOrientation::Positive);
        n1 = t0.ca;
        n4 = t0.bc;
        a = t0.a;
        b = t0.b;
        c = t0.c;
        if t1.ab == t0i {
            d = t1.c;
            n2 = t1.bc;
            n3 = t1.ca;
        } else if t1.bc == t0i {
            d = t1.a;
            n2 = t1.ca;
            n3 = t1.ab;
        } else {
            debug_assert!(t1.ca == t0i);
            d = t1.b;
            n2 = t1.ab;
            n3 = t1.bc;
        };
    }

    {
        let t0 = &mut mesh.triangles[t0i];
        t0.a = a;
        t0.b = d;
        t0.c = c;
        t0.ab = n2;
        t0.bc = t1i;
        t0.ca = n1;
    }

    {
        let t1 = &mut mesh.triangles[t1i];
        t1.a = d;
        t1.b = b;
        t1.c = c;
        t1.ab = n3;
        t1.bc = n4;
        t1.ca = t0i;
    }

    mesh.debug_triangle_orientation(t0i);
    mesh.debug_triangle_orientation(t1i);

    swap_neighbor(mesh, n2, t1i, t0i);
    swap_neighbor(mesh, n4, t0i, t1i);
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    //
    // Recall the flip diagram
    // ```text
    //  C    n4    B         C    n4    B
    //   ┌────────┐           ┌────────┐
    //   │       X│           │X       │
    //   │ t0   X │           │ X  t1  │
    //   │     X  │           │  X     │
    // n1│    X   │n3 ────► n1│   X    │n3
    //   │   X    │           │    X   │
    //   │  X     │           │     X  │
    //   │ X   t1 │           │ t0   X │
    //   │X       │           │       X│
    //   └────────┘           └────────┘
    //  A    n2    D         A    n2    D
    //  ```
    //  We want to test every rotation of t0 and t1
    #[test]
    fn flip_ab_1() {
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
        m.set_triangle_neighbors(n1, n1_1, t0, n1_2);
        let n2_1 = 22;
        let n2_2 = 23;
        m.set_triangle_neighbors(n2, n2_1, t1, n2_2);
        let n3_1 = 24;
        let n3_2 = 25;
        m.set_triangle_neighbors(n3, n3_1, t1, n3_2);
        let n4_1 = 26;
        let n4_2 = 27;
        m.set_triangle_neighbors(n4, n4_1, t0, n4_2);

        flip_ab(&mut m, t0);
        assert_eq!(
            m.triangle(t0),
            &Triangle {
                a: a,
                b: d,
                c: c,
                ab: n2,
                bc: t1,
                ca: n1
            }
        );
        assert_eq!(
            m.triangle(t1),
            &Triangle {
                a: d,
                b: b,
                c: c,
                ab: n3,
                bc: n4,
                ca: t0,
            }
        );
        assert_eq!(
            m.triangle(n1),
            &Triangle {
                a: n1p,
                b: a,
                c: c,
                ab: n1_1,
                bc: t0,
                ca: n1_2
            }
        );
        assert_eq!(
            m.triangle(n2),
            &Triangle {
                a: n2p,
                b: d,
                c: a,
                ab: n2_1,
                bc: t0,
                ca: n2_2
            }
        );
        assert_eq!(
            m.triangle(n3),
            &Triangle {
                a: n3p,
                b: b,
                c: d,
                ab: n3_1,
                bc: t1,
                ca: n3_2
            }
        );
        assert_eq!(
            m.triangle(n4),
            &Triangle {
                a: n4p,
                b: c,
                c: b,
                ab: n4_1,
                bc: t1,
                ca: n4_2
            }
        );
    }

    #[test]
    fn flip_ab_2() {}

    #[test]
    fn flip_ab_3() {}

    #[test]
    fn flip_bc() {}

    #[test]
    fn flip_ca() {}
}
