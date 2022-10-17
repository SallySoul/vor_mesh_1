use crate::geometry::*;

pub struct TriangleOrientationSimple {}

impl TriangleOrientationTest for TriangleOrientationSimple {
    fn new() -> Self {
        TriangleOrientationSimple {}
    }

    fn triangle_orientation(&self, a: &Vec2d, b: &Vec2d, c: &Vec2d) -> TriangleOrientation {
        let v0 = b - a;
        let v1 = c - a;
        let m = matrix![v0.x, v0.y;
                        v1.x, v1.y];
        let d = m.determinant();
        match d.partial_cmp(&0.0) {
            None => TriangleOrientation::NaN,
            Some(std::cmp::Ordering::Greater) => TriangleOrientation::Positive,
            Some(std::cmp::Ordering::Less) => TriangleOrientation::Negative,
            Some(std::cmp::Ordering::Equal) => TriangleOrientation::Degenerate,
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn all_cases() {
        let t = TriangleOrientationSimple::new();
        let a = vec2![0.0, 0.0];
        let b = vec2![1.0, 0.0];
        let c = vec2![0.0, 1.0];
        assert_eq!(
            t.triangle_orientation(&a, &b, &c),
            TriangleOrientation::Positive
        );

        let a = vec2![0.0, 0.0];
        let b = vec2![0.0, 1.0];
        let c = vec2![1.0, 0.0];
        assert_eq!(
            t.triangle_orientation(&a, &b, &c),
            TriangleOrientation::Negative
        );

        let a = vec2![0.0, 0.0];
        let b = vec2![0.0, 1.0];
        let c = vec2![0.0, 0.0];
        assert_eq!(
            t.triangle_orientation(&a, &b, &c),
            TriangleOrientation::Degenerate
        );
    }

    #[test]
    fn inf_test() {
        let t = TriangleOrientationSimple::new();
        let a = vec2![std::f64::NEG_INFINITY, std::f64::NEG_INFINITY];
        let b = vec2![std::f64::INFINITY, std::f64::NEG_INFINITY];
        let c = vec2![std::f64::NEG_INFINITY, std::f64::INFINITY];
        assert_eq!(t.triangle_orientation(&a, &b, &c), TriangleOrientation::NaN);
    }
}
