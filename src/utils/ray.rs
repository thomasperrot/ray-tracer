use crate::shapes::{Shape, Sphere};
use crate::utils::intersection::Intersection;
use crate::utils::vector::Vector;
use num_complex::ComplexFloat;

/// A ray
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn reflect(&mut self, intersection: &Intersection) {
        self.direction = (self.direction
            - intersection.normal * 2. * intersection.normal.dot(&self.direction))
        .normalize();
        self.origin = intersection.intersection;
    }

    pub fn refract(&mut self, intersection: &Intersection) {
        let is_ray_entering_the_shape = intersection.normal.dot(&self.direction) < 0.;
        if is_ray_entering_the_shape {
            let coeff = 1.
                - (1. - self.direction.dot(&intersection.normal).powi(2))
                    / intersection.shape.get_material().refractive_index.powi(2);
            self.direction = (self.direction
                * (1. / intersection.shape.get_material().refractive_index)
                - intersection.normal
                    * (self.direction.dot(&intersection.normal)
                        / intersection.shape.get_material().refractive_index
                        + coeff.sqrt()))
            .normalize();
            self.origin = intersection.intersection - intersection.normal * 0.05;
            // prevents a bug
        } else {
            // intersection.normal *= -1
            let inverted_normal = intersection.normal * -1.;
            let coeff = 1.
                - (1. - self.direction.dot(&inverted_normal).powi(2))
                    / intersection.shape.get_material().refractive_index.powi(2);
            if coeff < 0. {
                return self.reflect(intersection);
            }
            self.direction = (self.direction * intersection.shape.get_material().refractive_index
                - inverted_normal
                    * (self.direction.dot(&inverted_normal)
                        * intersection.shape.get_material().refractive_index
                        + coeff.sqrt()))
            .normalize();
            self.origin = intersection.intersection + intersection.normal * 0.05;
        }
    }
}
