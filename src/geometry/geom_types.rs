/// Macros
/// Prolly should make sure we don't re-export these?
pub use nalgebra::matrix;
pub use nalgebra::vector as vec2;
pub use nalgebra::vector as vec3;

/// Type Aliases
pub type Vec2d = nalgebra::base::Vector2<f64>;
pub type Mat4d = nalgebra::base::Matrix4<f64>;
