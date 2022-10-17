use crate::geometry::*;

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub ab: usize,
    pub bc: usize,
    pub ca: usize,
}

impl Triangle {
    pub fn new(a: usize, b: usize, c: usize) -> Triangle {
        Triangle {
            a,
            b,
            c,
            ab: 0,
            bc: 0,
            ca: 0,
        }
    }

    pub fn orientation <TC, TT, TO> (&self, mesh: &DelaunayMesh2d<TC, TT, TO>) -> TriangleOrientation
where
    TC: InCircleTest<Point = Vec2d>,
    TT: InTriangleTest,
    TO: TriangleOrientationTest,

    {
        let a = mesh.points[self.a];
        let b = mesh.points[self.b];
        let c = mesh.points[self.c];
        mesh.ot_test.triangle_orientation(&a, &b, &c)
    }
}
