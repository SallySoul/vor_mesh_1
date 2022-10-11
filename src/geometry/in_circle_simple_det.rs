use crate::geometry::*;

pub struct SimpleDetInCircle {}

impl InCircleTest for SimpleDetInCircle {
    type Point = Vec2d;

    fn new() -> Self {
        SimpleDetInCircle {}
    }

    fn in_circle(
        &self,
        a: &Self::Point,
        b: &Self::Point,
        c: &Self::Point,
        d: &Self::Point,
    ) -> InCircle {
        // TODO(rbentley): this could be reduced to 3x3 determinant
        let test_m: Mat4d = matrix![
            a.x, a.y, a.x * a.x + a.y * a.y, 1.0;
            b.x, b.y, b.x * b.x + b.y * b.y, 1.0;
            c.x, c.y, c.x * c.x + c.y * c.y, 1.0;
            d.x, d.y, d.x * d.x + d.y * d.y, 1.0f64
        ];

        let det = test_m.determinant();

        if (det - 0.0).abs() <= std::f64::EPSILON {
            InCircle::On
        } else if det > 0.0 {
            InCircle::In
        } else {
            InCircle::Out
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn simple_det() {
        let t = SimpleDetInCircle {};
        {
            let a = vec2![0.0, 0.0];
            let b = vec2![0.0, 0.0];
            let c = vec2![0.0, 0.0];
            let d = vec2![0.0, 0.0];
            let result = t.in_circle(&a, &b, &c, &d);
            assert_eq!(result, InCircle::On);
        }

        {
            let a = vec2![0.0, 1.0];
            let b = vec2![0.0, 0.0];
            let c = vec2![0.1, 0.0];
            let d = vec2![1.0, 1.0];
            let result = t.in_circle(&a, &b, &c, &d);
            assert_eq!(result, InCircle::Out);
        }

        {
            let a = vec2![0.0, 1.0];
            let b = vec2![0.0, 0.0];
            let c = vec2![0.1, 0.0];
            let d = vec2![0.1, 0.1];
            let result = t.in_circle(&a, &b, &c, &d);
            assert_eq!(result, InCircle::In);
        }
    }
}
