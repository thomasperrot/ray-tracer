use crate::constants::{BLACK, DIFFUSED, DIFFUSED_SAMPLES_COUNT, IMAGE_SIZE, MAX_BOUNCES};
use crate::shapes::Shape;
use crate::utils::intersection::Intersection;
use crate::utils::ray::Ray;
use crate::utils::vector::Vector;
use rand::prelude::*;
use std::f32::consts::{E, PI};
use tqdm::Iter;

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
}

impl Scene {
    pub fn generate_image(&self) {
        println!("[*] Generating image...");
        let mut imgbuf = image::ImageBuffer::new(IMAGE_SIZE.0, IMAGE_SIZE.1);
        let d = (IMAGE_SIZE.0 / 2) as f32 / (self.camera.fov / 2.).tan();
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut().tqdm() {
            let mut ray = self.generate_ray(y, x, d);
            let mut color = self.get_color(&mut ray, MAX_BOUNCES, false);
            if DIFFUSED {
                let mut diffused_color = [0f32; 3];
                for _ in 0..DIFFUSED_SAMPLES_COUNT {
                    ray = self.generate_ray(y, x, d);
                    let result = self.get_color(&mut ray, MAX_BOUNCES, true);
                    diffused_color[0] += result[0];
                    diffused_color[1] += result[1];
                    diffused_color[2] += result[2];
                }
                color[0] += diffused_color[0] / DIFFUSED_SAMPLES_COUNT as f32;
                color[1] += diffused_color[1] / DIFFUSED_SAMPLES_COUNT as f32;
                color[2] += diffused_color[2] / DIFFUSED_SAMPLES_COUNT as f32;
            }
            color[0] = color[0].powf(1. / 2.2);
            color[1] = color[1].powf(1. / 2.2);
            color[2] = color[2].powf(1. / 2.2);
            *pixel = image::Rgb([color[0] as u8, color[1] as u8, color[2] as u8]);
        }
        imgbuf.save("generated.png").unwrap();
        println!("[+] Successfully generated image");
    }
    fn generate_ray(&self, i: u32, j: u32, d: f32) -> Ray {
        let mut rng = rand::rng();
        let x: f32 = rng.random_range(0.0..1.);
        let y: f32 = rng.random_range(0.0..1.);
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

    fn get_color(&self, ray: &mut Ray, remaining_bounces: u8, diffused: bool) -> [f32; 3] {
        let mut result = BLACK;
        let diffused_part;
        let intersection = self.intersect(&ray);
        if intersection.is_none() {
            return BLACK;
        }
        let mut intersection = intersection.unwrap();

        // fixes a bug with specular materials
        intersection.intersection += intersection.normal * 0.0001;

        if intersection.intersection.square_norm() > 1_000_000. {
            return BLACK;
        }
        if intersection.shape.get_material().specular && remaining_bounces > 0 {
            ray.reflect(&intersection);
            return self.get_color(ray, remaining_bounces - 1, diffused);
        }
        if intersection.shape.get_material().refractive_index != 0. && remaining_bounces > 0 {
            ray.refract(&intersection);
            return self.get_color(ray, remaining_bounces - 1, diffused);
        }
        if diffused && remaining_bounces > 0 {
            diffuse(ray, &intersection);
            result = self.get_color(ray, remaining_bounces - 1, diffused);
        }
        let light_vector = self.light.origin - intersection.intersection;
        let light_vector_normalized = light_vector.normalize();
        let light_distance = light_vector.square_norm();
        if self.is_in_shadow(&intersection) {
            diffused_part = BLACK;
        } else {
            let light_value = light_vector_normalized.dot(&intersection.normal)
                * self.light.intensity as f32
                / (2. * PI * light_distance);
            let light_value = f32::max(light_value, 0.);
            let mat = intersection.shape.get_material();
            diffused_part = [
                light_value * mat.color[0] as f32,
                light_value * mat.color[1] as f32,
                light_value * mat.color[2] as f32,
            ];
        }
        [
            result[0] + diffused_part[0],
            result[1] + diffused_part[1],
            result[2] + diffused_part[2],
        ]
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

fn diffuse(ray: &mut Ray, intersection: &Intersection) {
    ray.origin = intersection.intersection + intersection.normal * 0.0001;
    ray.direction = {
        let mut rng = rand::rng();
        let intersect_dir_local = {
            let r1: f32 = rng.random_range(0.0..1.);
            let r2: f32 = rng.random_range(0.0..1.);
            Vector {
                x: (2. * PI * r1).cos() * (1. - r2).sqrt(),
                y: (2. * PI * r1).sin() * (1. - r2).sqrt(),
                z: r2.sqrt(),
            }
        };
        let tangent_1 = {
            let random_vec = Vector {
                x: rng.random_range(0.0..1.),
                y: rng.random_range(0.0..1.),
                z: rng.random_range(0.0..1.),
            };
            intersection.normal.cross(&random_vec)
        };
        let tangent_2 = intersection.normal.cross(&tangent_1);

        (tangent_1 * intersect_dir_local.x
            + tangent_2 * intersect_dir_local.y
            + intersection.normal * intersect_dir_local.z)
            .normalize()
    };
}
