use crate::shapes::Sphere;
use crate::utils::intersection::Intersection;
use crate::utils::vector::Vector;

/// A ray
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    fn reflect(sphere: Sphere, intersection: Intersection) {}
}
