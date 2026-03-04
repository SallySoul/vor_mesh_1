pub use crate::geometry::*;

pub struct CircumCircle {
    pub center: Vec2d,
    pub radius: f64,
}

pub fn circumcircle<TC, TT, TO>(mesh: &DelaunayMesh2d<TC, TT, TO>, ti: usize) -> CircumCircle 
    where
        TC: InCircleTest<Point = Vec2d>,
        TT: InTriangleTest,
        TO: TriangleOrientationTest,
    {
        let t = &mesh.triangles[ti];
        // https://en.wikipedia.org/wiki/Circumcircle
        let a = mesh.points[t.a];
        let b = mesh.points[t.b];
        let c = mesh.points[t.c];

        let s_x_m = matrix![
            a.norm_squared(), a.y, 1.0;
            b.norm_squared(), b.y, 1.0;
            c.norm_squared(), c.y, 1.0;
        ];
        let s_x = 0.5 * s_x_m.determinant();

        let s_y_m = matrix![
            a.x, a.norm_squared(), 1.0;
            b.x, b.norm_squared(), 1.0;
            c.x, c.norm_squared(), 1.0;
        ];
        let s_y = 0.5 * s_y_m.determinant();

        let l1_m = matrix![
            a.x, a.y, 1.0;
            b.x, b.y, 1.0;
            c.x, c.y, 1.0;
        ];
        let l1 = l1_m.determinant();

        let l2_m = matrix![
            a.x, a.y, a.norm_squared();
            b.x, b.y, b.norm_squared();
            c.x, c.y, b.norm_squared();
        ];
        let l2 = l2_m.determinant();

        let s = vec2![s_x, s_y];
        let r = (l2 / l1 + s.norm_squared() / (l1 * l1)).sqrt(); 

        CircumCircle {
            center: s / l1,
            radius: r,
        }
    }
