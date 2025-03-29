use crate::constants::{IMAGE_SIZE, MAX_BOUNCES};
use crate::shapes::{Shape, Sphere};
use crate::utils::intersection::Intersection;
use crate::utils::ray::Ray;
use crate::utils::vector::Vector;
use rand::prelude::*;
use std::cmp;
use std::f32::consts::{E, PI};
use std::ops::Deref;

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
            let mut ray = self.generate_ray(y, x, d);
            let color = self.get_color(&mut ray, MAX_BOUNCES);
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

    fn get_color(&self, ray: &mut Ray, remaining_bounces: u8) -> image::Rgb<u8> {
        let black = image::Rgb([0, 0, 0]);
        let intersection = self.intersect(&ray);
        if intersection.is_none() {
            return black;
        }
        let mut intersection = intersection.unwrap();

        // fixes a bug with specular materials
        intersection.intersection += intersection.normal * 0.0001;

        if intersection.intersection.square_norm() > 1_000_000. {
            return black;
        }
        if intersection.shape.get_material().specular && remaining_bounces > 0 {
            ray.reflect(&intersection);
            return self.get_color(ray, remaining_bounces - 1);
        }
        if intersection.shape.get_material().refractive_index != 0. && remaining_bounces > 0 {
            ray.refract(&intersection);
            return self.get_color(ray, remaining_bounces - 1);
        }
        let light_vector = self.light.origin - intersection.intersection;
        let light_vector_normalized = light_vector.normalize();
        let light_distance = light_vector.square_norm();
        if self.is_in_shadow(&intersection) {
            return black;
        }
        let light_value = light_vector_normalized.dot(&intersection.normal)
            * self.light.intensity as f32
            / (2. * PI * light_distance);
        let light_value = f32::max(light_value, 0.);
        let mat = intersection.shape.get_material();
        image::Rgb([
            (light_value.trunc() * mat.color[0] as f32 / 256.) as u8,
            (light_value.trunc() * mat.color[1] as f32 / 256.) as u8,
            (light_value.trunc() * mat.color[2] as f32 / 256.) as u8,
        ])
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.shapes
            .iter()
            .filter_map(|shape| shape.get_intersection(&ray))
            .min_by_key(|intersection| intersection.d.round() as u32)
    }

    fn is_in_shadow(&self, intersection: &Intersection) -> bool {
        let v_light = self.light.origin - intersection.intersection;
        let v_light_normalized = (self.light.origin - intersection.intersection).normalize();
        let light_distance = v_light.square_norm();

        let tmp_ray = Ray {
            origin: intersection.intersection,
            direction: v_light_normalized,
        };
        self.shapes
            .iter()
            .filter_map(|shape| shape.get_intersection(&tmp_ray))
            .any(|intersection| intersection.d.powi(2) < light_distance)
    }
}
