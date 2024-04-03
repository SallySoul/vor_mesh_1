use crate::geometry::*;

pub struct InTriangleSimple;

impl InTriangleTest for InTriangleSimple {
    fn new() -> Self {
        InTriangleSimple {}
    }

    // https://blackpawn.com/texts/pointinpoly/
    fn in_triangle(&self, a: &Vec2d, b: &Vec2d, c: &Vec2d, p: &Vec2d) -> InTriangle {
        // Compute vectors
        let v0 = c - a;
        let v1 = b - a;
        let v2 = p - a;

        // Compute dot products
        let dot00 = v0.dot(&v0);
        let dot01 = v0.dot(&v1);
        let dot02 = v0.dot(&v2);
        let dot11 = v1.dot(&v1);
        let dot12 = v1.dot(&v2);

        // Compute barycentric coordinates
        let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

        let result = if u == 0.0 && v == 0.0 {
            InTriangle::OnA
        } else if u == 1.0 && v == 0.0 {
            InTriangle::OnC
        } else if u == 0.0 && v == 1.0 {
            InTriangle::OnB
        } else if v < 0.0 {
            InTriangle::OutsideCA
        } else if u < 0.0 {
            InTriangle::OutsideAB
        } else if u > 0.0 && u < 1.0 && v == 0.0 {
            InTriangle::OnAB
        } else if u + v == 1.0 {
            InTriangle::OnBC
        } else if u == 0.0 && v > 0.0 && v < 1.0 {
            InTriangle::OnCA
        } else if u > 0.0 && v > 0.0 && u + v < 1.0 {
            InTriangle::In
        } else if u + v > 1.0 {
            InTriangle::OutsideBC
        } else {
            panic!("Bad InTriangle test?, u: {}, v: {}", u, v);
        };

        println!("u: {}, v: {}, r: {:?}", u, v, result);

        result
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn simple_cases_1() {
        let t = InTriangleSimple::new();
        let o = TriangleOrientationSimple::new();

        let a = vec2![1.0, 0.0];
        let b = vec2![4.0, 0.0];
        let c = vec2![4.0, 4.0];
        assert_eq!(
            o.triangle_orientation(&a, &b, &c),
            TriangleOrientation::Positive
        );
        assert_eq!(t.in_triangle(&a, &b, &c, &a), InTriangle::OnA);
        assert_eq!(t.in_triangle(&a, &b, &c, &b), InTriangle::OnB);
        assert_eq!(t.in_triangle(&a, &b, &c, &c), InTriangle::OnC);

        for p in [vec2![1.1, 0.01], vec2![3.0, 1.0]] {
            assert_eq!(t.in_triangle(&a, &b, &c, &p), InTriangle::In, "{:?}", p);
        }

        for p in [vec2![3.0, -4.0], vec2![1.0, -1.0]] {
            assert_eq!(
                t.in_triangle(&a, &b, &c, &p),
                InTriangle::OutsideAB,
                "{:?}",
                p
            );
        }

        for p in [vec2![6.0, 5.0], vec2![15.0, 7.0]] {
            assert_eq!(
                t.in_triangle(&a, &b, &c, &p),
                InTriangle::OutsideBC,
                "{:?}",
                p
            );
        }

        for p in [vec2![0.0, 4.0], vec2![-5.0, -1.0]] {
            assert_eq!(
                t.in_triangle(&a, &b, &c, &p),
                InTriangle::OutsideCA,
                "{:?}",
                p
            );
        }
    }

    #[test]
    fn simple_cases_2() {
        let t = InTriangleSimple::new();
        let o = TriangleOrientationSimple::new();

        let a = vec2![-2.0, -3.0];
        let b = vec2![3.0, -1.0];
        let c = vec2![1.0, 4.0];
        assert_eq!(
            o.triangle_orientation(&a, &b, &c),
            TriangleOrientation::Positive
        );

        for p in [vec2![1.1, 1.01], vec2![0.0, -1.0], vec2![2.0, 0.0]] {
            assert_eq!(t.in_triangle(&a, &b, &c, &p), InTriangle::In, "{:?}", p);
        }

        for p in [vec2![0.0, -4.0], vec2![-1.9, -3.4]] {
            assert_eq!(
                t.in_triangle(&a, &b, &c, &p),
                InTriangle::OutsideAB,
                "{:?}",
                p
            );
        }

        for p in [vec2![3.0, 1.0], vec2![2.0, 2.0]] {
            assert_eq!(
                t.in_triangle(&a, &b, &c, &p),
                InTriangle::OutsideBC,
                "{:?}",
                p
            );
        }

        for p in [vec2![-1.0, 1.0], vec2![-4.0, -2.0]] {
            assert_eq!(
                t.in_triangle(&a, &b, &c, &p),
                InTriangle::OutsideCA,
                "{:?}",
                p
            );
        }
    }
}
