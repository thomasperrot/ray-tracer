use crate::shapes::{Shape, Sphere};
use crate::utils::intersection::Intersection;
use crate::utils::vector::Vector;

/// A ray
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn reflect(&mut self, intersection: &Intersection) {
        self.direction = ((self.direction - intersection.normal * 2. * intersection.normal.dot(&self.direction))).normalize();
        self.origin = intersection.intersection;
    }

}
