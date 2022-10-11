use crate::geometry::*;

pub enum DelaunayError {
    /// A duplicated position was inserted for the point at original_index
    InsertedDuplicatePoint { original_index: usize },
}

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
    fn new(a: usize, b: usize, c: usize) -> Triangle {
        Triangle {
            a,
            b,
            c,
            ab: 0,
            bc: 0,
            ca: 0,
        }
    }
}

pub struct DelaunayMesh2d<
    TC: InCircleTest<Point = Vec2d>,
    TT: InTriangleTest,
    TO: TriangleOrientationTest,
> {
    pub ic_test: TC,
    pub it_test: TT,
    pub ot_test: TO,
    pub points: Vec<Vec2d>,
    pub triangles: Vec<Triangle>,
}

impl<TC: InCircleTest<Point = Vec2d>, TT: InTriangleTest, TO: TriangleOrientationTest>
    DelaunayMesh2d<TC, TT, TO>
{
    pub fn empty() -> Self {
        DelaunayMesh2d {
            ic_test: TC::new(),
            it_test: TT::new(),
            ot_test: TO::new(),
            points: vec![Vec2d::from_element(std::f64::NAN)],
            triangles: vec![Triangle::new(0, 0, 0)],
        }
    }

    pub fn bounded(a: Vec2d, b: Vec2d, c: Vec2d) -> Self {
        DelaunayMesh2d {
            ic_test: TC::new(),
            it_test: TT::new(),
            ot_test: TO::new(),
            points: vec![a, b, c],
            triangles: vec![Triangle::new(0, 1, 2)],
        }
    }

    pub fn debug_triangle_orientation(&self, ti: usize) {
        let t = self.triangles[ti];
        let a = &self.points[t.a];
        let b = &self.points[t.b];
        let c = &self.points[t.c];
        debug_assert_eq!(
            self.ot_test.triangle_orientation(a, b, c),
            TriangleOrientation::Positive
        );
    }

    pub fn add_point(&mut self, p: Vec2d) -> usize {
        let i = self.points.len();
        self.points.push(p);
        i
    }

    pub fn add_triangle(&mut self, a: usize, b: usize, c: usize) -> usize {
        let i = self.triangles.len();
        self.triangles.push(Triangle::new(a, b, c));
        self.debug_triangle_orientation(i);
        i
    }

    pub fn insert_point(&mut self, pi: usize, ti: usize) -> Result<usize, DelaunayError> {
        let p = &self.points[pi];

        // Find Triangle
        let mut ti = ti;
        loop {
            let t = &self.triangles[ti];
            let a = &self.points[t.a];
            let b = &self.points[t.b];
            let c = &self.points[t.c];
            match self.it_test.in_triangle(a, b, c, p) {
                InTriangle::In => {
                    self.insert_in_triangle(ti, pi);
                    break;
                }
                InTriangle::OnAB => {
                    self.insert_on_triangle_ab(ti, pi);
                    break;
                }
                InTriangle::OnBC => {
                    self.insert_on_triangle_bc(ti, pi);
                    break;
                }
                InTriangle::OnCA => {
                    self.insert_on_triangle_ca(ti, pi);
                    break;
                }
                InTriangle::OnA => {
                    return Err(DelaunayError::InsertedDuplicatePoint {
                        original_index: t.a,
                    });
                }
                InTriangle::OnB => {
                    return Err(DelaunayError::InsertedDuplicatePoint {
                        original_index: t.b,
                    });
                }
                InTriangle::OnC => {
                    return Err(DelaunayError::InsertedDuplicatePoint {
                        original_index: t.b,
                    });
                }
                InTriangle::OutsideAB => {
                    ti = t.ab;
                }
                InTriangle::OutsideBC => {
                    ti = t.bc;
                }
                InTriangle::OutsideCA => {
                    ti = t.ca;
                }
            }
        }

        Ok(ti)
    }

    pub fn insert_on_triangle_ab(&mut self, _t: usize, _p: usize) {}

    pub fn insert_on_triangle_bc(&mut self, _t: usize, _p: usize) {}

    pub fn insert_on_triangle_ca(&mut self, _t: usize, _p: usize) {}

    pub fn insert_in_triangle(&mut self, t: usize, p: usize) {
        // Get points setup on all triangles
        let t0 = self.triangles[t];
        self.triangles[t].c = p;
        let t2 = self.add_triangle(p, t0.b, t0.c);
        let t3 = self.add_triangle(t0.a, p, t0.c);

        // Setup neighbors
        self.triangles[t].bc = t2;
        self.triangles[t].ca = t3;

        self.triangles[t2].ab = t;
        self.triangles[t2].bc = t0.bc;
        self.triangles[t2].ca = t3;

        self.triangles[t3].ab = t;
        self.triangles[t3].bc = t2;
        self.triangles[t3].ca = t0.ca;
    }
}
