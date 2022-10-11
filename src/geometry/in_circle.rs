#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum InCircle {
    In,
    Out,
    On,
}

pub trait InCircleTest {
    type Point;
    fn new() -> Self;

    fn in_circle(
        &self,
        a: &Self::Point,
        b: &Self::Point,
        c: &Self::Point,
        d: &Self::Point,
    ) -> InCircle;
}
