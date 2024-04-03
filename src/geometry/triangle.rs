use crate::geometry::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Triangle {
    /// Index of point a
    pub a: usize,

    /// Index of point b
    pub b: usize,

    /// Index of point c
    pub c: usize,

    /// Index of triangle sharing edge ab
    pub ab: usize,

    /// Index of triangle sharing edge bc
    pub bc: usize,

    /// Index of triangle sharing edge ca
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

    pub fn orientation<TC, TT, TO>(&self, mesh: &DelaunayMesh2d<TC, TT, TO>) -> TriangleOrientation
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
