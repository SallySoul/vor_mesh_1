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
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn flip_ab() {
        let mut m = Simple2DMesh::empty();
        let a = m.add_point(vec2![0.0, 0.0]);
        let b = m.add_point(vec2![1.0, 1.0]);
        let c = m.add_point(vec2![0.0, 1.0]);
        let d = m.add_point(vec2![1.0, 0.0]);
        let n1 = 5;
        let n2 = 7 ;
        let n3 = 11;
        let n4 = 13;
        let t0 = 0;
        let t1 = 1;
        assert_eq!(t0, m.add_triangle(a, b, c));
        m.triangles[0].ab = t1;
        m.triangles[0].bc = n4;
        m.triangles[0].ca = n1;
        assert_eq!(t1, m.add_triangle(d, b, a));
        m.triangles[1].ab = n3;
        m.triangles[1].bc = t0;
        m.triangles[1].ca = n2;
        super::flip_ab(&mut m, 0);
        assert_eq!(m.points.len(), 4);
        assert_eq!(m.triangles.len(), 2);
        assert_eq!(m.triangles[0].a, a);
        assert_eq!(m.triangles[0].b, d);
        assert_eq!(m.triangles[0].c, c);
        assert_eq!(m.triangles[0].ab, n2);
        assert_eq!(m.triangles[0].bc, t1);
        assert_eq!(m.triangles[0].ca, n1);
        assert_eq!(m.triangles[1].a, d);
        assert_eq!(m.triangles[1].b, b);
        assert_eq!(m.triangles[1].c, c);
        assert_eq!(m.triangles[1].ab, n3);
        assert_eq!(m.triangles[1].bc, n4);
        assert_eq!(m.triangles[1].ca, t0);
    }

    #[test]
    fn flip_bc() {
        let mut m = Simple2DMesh::empty();
        let a = m.add_point(vec2![0.0, 0.0]);
        let b = m.add_point(vec2![1.0, 1.0]);
        let c = m.add_point(vec2![0.0, 1.0]);
        let d = m.add_point(vec2![1.0, 0.0]);
        let n1 = 5;
        let n2 = 7 ;
        let n3 = 11;
        let n4 = 13;
        let t0 = 0;
        let t1 = 1;
        m.add_triangle(c, a, b);
        m.triangles[0].ab = n1;
        m.triangles[0].bc = t1;
        m.triangles[0].ca = n4;
        m.add_triangle(b, a, d);
        m.triangles[1].ab = t0;
        m.triangles[1].bc = n2;
        m.triangles[1].ca = n3;
        super::flip_bc(&mut m, 0);
        assert_eq!(m.points.len(), 4);
        assert_eq!(m.triangles.len(), 2);
        assert_eq!(m.triangles[0].a, a);
        assert_eq!(m.triangles[0].b, d);
        assert_eq!(m.triangles[0].c, c);
        assert_eq!(m.triangles[0].ab, n2);
        assert_eq!(m.triangles[0].bc, t1);
        assert_eq!(m.triangles[0].ca, n1);
        assert_eq!(m.triangles[1].a, d);
        assert_eq!(m.triangles[1].b, b);
        assert_eq!(m.triangles[1].c, c);
        assert_eq!(m.triangles[1].ab, n3);
        assert_eq!(m.triangles[1].bc, n4);
        assert_eq!(m.triangles[1].ca, t0);
    }

    #[test]
    fn flip_ca() {
        let mut m = Simple2DMesh::empty();
        let a = m.add_point(vec2![0.0, 0.0]);
        let b = m.add_point(vec2![1.0, 1.0]);
        let c = m.add_point(vec2![0.0, 1.0]);
        let d = m.add_point(vec2![1.0, 0.0]);
        let n1 = 5;
        let n2 = 7 ;
        let n3 = 11;
        let n4 = 13;
        let t0 = 0;
        let t1 = 1;
        m.add_triangle(b, c, a);
        m.triangles[0].ab = n4;
        m.triangles[0].bc = n1;
        m.triangles[0].ca = t1;
        m.add_triangle(a, d, b);
        m.triangles[1].ab = n2;
        m.triangles[1].bc = n3;
        m.triangles[1].ca = t0;
        super::flip_ca(&mut m, 0);
        assert_eq!(m.points.len(), 4);
        assert_eq!(m.triangles.len(), 2);
        assert_eq!(m.triangles[0].a, a);
        assert_eq!(m.triangles[0].b, d);
        assert_eq!(m.triangles[0].c, c);
        assert_eq!(m.triangles[0].ab, n2);
        assert_eq!(m.triangles[0].bc, t1);
        assert_eq!(m.triangles[0].ca, n1);
        assert_eq!(m.triangles[1].a, d);
        assert_eq!(m.triangles[1].b, b);
        assert_eq!(m.triangles[1].c, c);
        assert_eq!(m.triangles[1].ab, n3);
        assert_eq!(m.triangles[1].bc, n4);
        assert_eq!(m.triangles[1].ca, t0);
    }
}
