use crate::geometry::*;

#[derive(Debug)]
pub enum DelaunayError {
    /// A duplicated position was inserted for the point at original_index
    InsertedDuplicatePoint { original_index: usize },
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

/// NOTE: Think we need to separate triangle mesh and delaunay builder
impl<TC: InCircleTest<Point = Vec2d>, TT: InTriangleTest, TO: TriangleOrientationTest>
    DelaunayMesh2d<TC, TT, TO>
{
    /// No points or triangles.
    /// WARNING: Only use this if you're sure you know what you're doing.
    /// More likely bounded is the better option for you
    pub fn empty() -> Self {
        DelaunayMesh2d {
            ic_test: TC::new(),
            it_test: TT::new(),
            ot_test: TO::new(),
            points: Vec::new(),
            triangles: Vec::new(),
        }
    }

    pub fn bounded(a: Vec2d, b: Vec2d, c: Vec2d) -> Self {
        DelaunayMesh2d {
            ic_test: TC::new(),
            it_test: TT::new(),
            ot_test: TO::new(),
            points: vec![vec2![std::f64::NAN, std::f64::NAN], a, b, c],
            triangles: vec![Triangle::new(0, 0, 0), Triangle::new(1, 2, 3)],
        }
    }

    #[track_caller]
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

    pub fn in_circle(&self, ti: usize, p: &Vec2d) -> bool {
        let t = self.triangles[ti];
        let a = &self.points[t.a];
        let b = &self.points[t.b];
        let c = &self.points[t.c];
        self.ic_test.in_circle(a, b, c, p) == InCircle::In
    }

    pub fn add_point(&mut self, p: Vec2d) -> usize {
        let i = self.points.len();
        self.points.push(p);
        i
    }

    /// Get a reference to Triangle `ti`.
    pub fn triangle(&self, ti: usize) -> &Triangle {
        &self.triangles[ti]
    }

    /// Get a mutable reference to Triangle `ti`
    pub fn mut_triangle(&mut self, ti: usize) -> &mut Triangle {
        &mut self.triangles[ti]
    }

    #[track_caller]
    pub fn add_triangle(&mut self, a: usize, b: usize, c: usize) -> usize {
        let i = self.triangles.len();
        self.triangles.push(Triangle::new(a, b, c));
        self.debug_triangle_orientation(i);
        i
    }

    #[track_caller]
    pub fn set_triangle_neighbors(&mut self, ti: usize, ab: usize, bc: usize, ca: usize) {
        self.triangles[ti].ab = ab;
        self.triangles[ti].bc = bc;
        self.triangles[ti].ca = ca;
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

    pub fn insert_on_triangle_ab(&mut self, _t: usize, _p: usize) {
        panic!("Not implemented");
    }

    pub fn insert_on_triangle_bc(&mut self, _t: usize, _p: usize) {
        panic!("Not implemented");
    }

    pub fn insert_on_triangle_ca(&mut self, _t: usize, _p: usize) {
        panic!("Not implemented");
    }

    pub fn insert_in_triangle(&mut self, t: usize, p: usize) {
        // Get points setup on all triangles
        let a = self.triangles[t].a;
        let b = self.triangles[t].b;
        let c = self.triangles[t].c;
        let n2 = self.triangles[t].bc;
        let n3 = self.triangles[t].ca;

        // Update triangle vertices
        self.triangles[t].c = p;
        let t1 = self.add_triangle(p, b, c);
        let t2 = self.add_triangle(a, p, c);

        // Setup neighbors
        self.triangles[t].bc = t1;
        self.triangles[t].ca = t2;
        self.triangles[t1].ab = t;
        self.triangles[t1].bc = n2;
        self.triangles[t1].ca = t2;
        swap_neighbor(self, n2, t, t1);
        self.triangles[t2].ab = t;
        self.triangles[t2].bc = t1;
        self.triangles[t2].ca = n3;
        swap_neighbor(self, n3, t, t2);

        // Perform swap tests
        swap_test_ab(self, t);
        swap_test_bc(self, t1);
        swap_test_ca(self, t2);
    }

    pub fn rotate_triangle(&mut self, ti: usize) {
        let t = self.triangles[ti];
        self.triangles[ti].a = t.b;
        self.triangles[ti].b = t.c;
        self.triangles[ti].c = t.a;
        self.triangles[ti].ab = t.bc;
        self.triangles[ti].bc = t.ca;
        self.triangles[ti].ca = t.ab;
    }

    pub fn triangle_center(&self, ti: usize) -> Vec2d {
        let a = self.points[self.triangles[ti].a];
        let b = self.points[self.triangles[ti].b];
        let c = self.points[self.triangles[ti].c];
        (a + b + c) / 3.0
    }
}
