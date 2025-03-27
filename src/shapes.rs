use crate::materials::Material;
use crate::utils::intersection::Intersection;
use crate::utils::ray::Ray;
use crate::utils::vector::Vector;
use std::fmt::Debug;

pub trait Shape {
    /// Return the intersection between the shape and a ray
    fn get_intersection(&self, ray: &Ray) -> Option<Intersection>;

    /// Return the normal vector to the shape at a given point
    fn get_normal(&self, v: &Vector) -> Vector;

    fn get_material(&self) -> Material;
}

impl Debug for dyn Shape {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Shape")
    }
}

pub struct Sphere {
    pub origin: Vector,
    pub radius: f32,
    pub material: Material,
}

impl Shape for Sphere {
    fn get_intersection(&self, ray: &Ray) -> Option<Intersection> {
        let a = 1.;
        let b = 2.0 * ray.direction.dot(&(ray.origin - self.origin));
        let c = (ray.origin - self.origin).square_norm() - self.radius.powi(2);

        let delta = b * b - 4. * a * c;
        if delta > 0. {
            let tmin = (-b - delta.sqrt()) / (2.0 * a);
            let tmax = (-b + delta.sqrt()) / (2.0 * a);
            if tmax > 0. {
                let d = if tmin > 0. { tmin } else { tmax };
                let intersection = ray.direction * d + ray.origin;
                // see https://github.com/thomasperrot/python_graphique/blob/master/decors/Scene.py#L141C57-L141C58. Maybe todo
                Some(Intersection {
                    d,
                    intersection,
                    normal: self.get_normal(&intersection),
                    shape: self,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_normal(&self, v: &Vector) -> Vector {
        (*v - self.origin).normalize()
    }

    fn get_material(&self) -> Material {
        self.material
    }
}

/// An hyperboloid structure
/// cf. https://en.wikipedia.org/wiki/Hyperboloid
pub struct Hyperboloid {
    pub origin: Vector,
    /// The a, b and c coefficients
    pub coefficients: (f32, f32, f32),
    pub material: Material,
}

impl Shape for Hyperboloid {
    fn get_intersection(&self, ray: &Ray) -> Option<Intersection> {
        todo!()
    }
    fn get_normal(&self, v: &Vector) -> Vector {
        Vector {
            x: (v.x - self.origin.x) / (self.coefficients.0.powi(2)),
            y: (v.y - self.origin.y) / (self.coefficients.1.powi(2)),
            z: (v.z - self.origin.z) / (self.coefficients.2.powi(2)),
        }
        .normalize()
    }

    fn get_material(&self) -> Material {
        self.material
    }
}
