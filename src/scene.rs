use crate::constants::IMAGE_SIZE;
use crate::shapes::{Shape, Sphere};
use crate::utils::intersection::Intersection;
use crate::utils::ray::Ray;
use crate::utils::vector::Vector;
use rand::prelude::*;
use std::cmp;
use std::f32::consts::{E, PI};

pub struct Camera {
    pub point: Vector,
    /// field of vision, which is the opening angle, in radians
    pub fov: f32,
    pub direction: Vector,
    pub up: Vector,
    pub right: Vector,
}

pub struct Light {
    pub origin: Vector,
    pub intensity: u32,
}

pub struct Scene {
    pub shapes: Vec<Box<dyn Shape>>,
    pub light: Light,
    pub camera: Camera,
    rng: ThreadRng,
}

impl Scene {
    pub fn build(shapes: Vec<Box<dyn Shape>>, light: Light, camera: Camera) -> Self {
        let mut rng = rand::rng();
        Scene {
            shapes,
            light,
            camera,
            rng,
        }
    }
    pub fn generate_image(&mut self) {
        let mut imgbuf = image::ImageBuffer::new(IMAGE_SIZE.0, IMAGE_SIZE.1);
        let d = (IMAGE_SIZE.0 / 2) as f32 / (self.camera.fov / 2.).tan();
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let ray = self.generate_ray(x, y, d);
            let color = self.get_color(ray);
            *pixel = color;
        }
        imgbuf.save("generated.png").unwrap();
    }
    fn generate_ray(&mut self, i: u32, j: u32, d: f32) -> Ray {
        let x: f32 = self.rng.random_range(0.0..1.);
        let y: f32 = self.rng.random_range(0.0..1.);
        let r = (-2. * x.log(E)).sqrt();
        let u = r * (2. * PI * y).cos() / 2.;
        let v = r * (2. * PI * y).sin() / 2.;
        Ray {
            origin: self.camera.point,
            direction: (self.camera.right * (j as f32 - (IMAGE_SIZE.0 as f32) / 2. - 0.5 + u)
                + self.camera.up * (i as f32 - IMAGE_SIZE.1 as f32 / 2. - 0.5 + v)
                + self.camera.direction * (-d))
                .normalize(),
        }
    }

    fn get_color(&self, ray: Ray) -> image::Rgb<u8> {
        let black = image::Rgb([0, 0, 0]);
        let intersection = self.intersect(&ray);
        if intersection.is_none() {
            return black;
        }
        let intersection = intersection.unwrap();
        if intersection.intersection.square_norm() > 1_000_000. {
            return black;
        }
        let light_vector = self.light.origin - intersection.intersection;
        let light_vector_normalized = light_vector.normalize();
        let light_distance = light_vector.square_norm();
        let light_value = light_vector_normalized.dot(&intersection.normal)
            * self.light.intensity as f32
            / (2. * PI * light_distance);
        let light_value = f32::max(light_value, 0.);
        image::Rgb([
            (light_value).trunc() as u8,
            (light_value).trunc() as u8,
            (light_value).trunc() as u8,
        ])
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.shapes
            .iter()
            .map(|shape| shape.get_intersection(&ray))
            .filter(|intersection| intersection.is_some())
            .map(|intersection| intersection.unwrap())
            .min_by_key(|intersection| intersection.d.round() as u32)
    }
}
