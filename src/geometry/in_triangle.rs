use crate::geometry::geom_types::Vec2d;

/// Possible relations between a triangle `ABC` and a point `P`
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum InTriangle {
    /// `P` is inside Triangle
    In,

    /// `P` is a duplicate of `A`
    OnA,

    /// `P` is a duplicate of `B`
    OnB,

    /// `P` is a duplicate of `C`
    OnC,

    /// `P` is on edge `AB`
    OnAB,

    /// `P` is on edge `BC`
    OnBC,

    /// `P` is on dege `CA`
    OnCA,

    /// `P` is outside `AB`
    OutsideAB,

    /// `P` is outside `BC`
    OutsideBC,

    /// `P` is outside `CA`
    OutsideCA,
}

pub trait InTriangleTest {
    fn new() -> Self;
    fn in_triangle(&self, a: &Vec2d, b: &Vec2d, c: &Vec2d, p: &Vec2d) -> InTriangle;
}
