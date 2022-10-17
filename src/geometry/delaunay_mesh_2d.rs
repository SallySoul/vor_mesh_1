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
            points: vec![a, b, c],
            triangles: vec![Triangle::new(0, 1, 2)],
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

    pub fn add_point(&mut self, p: Vec2d) -> usize {
        let i = self.points.len();
        self.points.push(p);
        i
    }

    #[track_caller]
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
    pub fn flip_ab(&mut self, t0i: usize) {
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
            let t0 = &self.triangles[t0i];
            t1i = t0.ab;
            let t1 = &self.triangles[t1i];
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
            let t0 = &mut self.triangles[t0i];
            t0.a = a;
            t0.b = d;
            t0.c = c;
            t0.ab = n2;
            t0.bc = t1i;
            t0.ca = n1;
        }

        {
            let t1 = &mut self.triangles[t1i];
            t1.a = d;
            t1.b = b;
            t1.c = c; 
            t1.ab = n3;
            t1.bc = n4;
            t1.ca = t1i;
        }
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

#[cfg(test)]
mod unit_tests {
    use super::*;

}
