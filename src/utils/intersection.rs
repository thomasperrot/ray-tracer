use crate::shapes::Shape;
use crate::utils::vector::Vector;

/// The intersection of a shape with the ray line
pub struct Intersection<'a> {
    pub d: f32,
    /// the point of intersection
    pub intersection: Vector,
    /// the normal to the point of intersection
    pub normal: Vector,
    pub shape: &'a (dyn Shape),
}
