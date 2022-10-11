use crate::geometry::geom_types::Vec2d;

/// Possible orientations for a triangle `T`
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TriangleOrientation {
    /// `T` is positively oriented
    Positive,

    /// `T` is negativley oriented
    Negative,

    /// `T` is a degenerate triangle
    Degenerate,

    /// Test involved a NaN
    NaN,
}

pub trait TriangleOrientationTest {
    fn new() -> Self;
    fn triangle_orientation(&self, a: &Vec2d, b: &Vec2d, c: &Vec2d) -> TriangleOrientation;
}
