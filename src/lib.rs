use crate::constants::IMAGE_SIZE;
use crate::materials::Material;
use crate::scene::{Camera, Light, Scene};
use crate::shapes::{Hyperboloid, Sphere};
use crate::utils::vector::Vector;
use image::Rgb;
use std::f32::consts::PI;

mod constants;
mod materials;
mod scene;
mod shapes;
mod utils;

pub fn make_image() {
    let light = Light {
        origin: Vector {
            x: -10.,
            y: -20.,
            z: 50.,
        },
        intensity: 4_000_000,
    };
    let camera = Camera {
        point: Vector {
            x: 0.,
            y: 0.,
            z: 55.,
        },
        fov: 90. * PI / 180.,
        direction: Vector {
            x: 0.,
            y: 0.,
            z: 1.,
        }
        .normalize(),
        up: Vector {
            x: 0.,
            y: 1.,
            z: 0.,
        }
        .normalize(),
        right: Vector {
            x: 1.,
            y: 0.,
            z: 0.,
        }
        .normalize(),
    };
    let opaque_red = Material {
        color: Rgb([255, 0, 0]),
        specular: false,
        refractive_index: 0.,
    };
    let opaque_green = Material {
        color: Rgb([0, 255, 0]),
        specular: false,
        refractive_index: 0.,
    };
    let opaque_blue = Material {
        color: Rgb([0, 0, 255]),
        specular: false,
        refractive_index: 0.,
    };
    let opaque_cyan = Material {
        color: Rgb([0, 255, 255]),
        specular: false,
        refractive_index: 0.,
    };
    let opaque_white = Material {
        color: Rgb([255, 255, 255]),
        specular: false,
        refractive_index: 0.,
    };
    let opaque_yellow = Material {
        color: Rgb([255, 255, 0]),
        specular: false,
        refractive_index: 0.,
    };
    let reflective = Material {
        color: Rgb([255, 255, 255]),
        specular: true,
        refractive_index: 0.,
    };
    let transparent = Material {
        color: Rgb([255, 255, 255]),
        specular: false,
        refractive_index: 2.3,
    };

    let main = Sphere {
        origin: Vector {
            x: 0.,
            y: -5.,
            z: 25.,
        },
        radius: 10.,
        // material: reflective,
        // material: opaque_white,
        material: transparent,
    };
    let hyperboloid = Hyperboloid {
        origin: Vector {
            x: 0.,
            y: 0.,
            z: -10.,
        },
        coefficients: (0.5, 1., 0.5),
        material: opaque_white,
    };
    let back = Sphere {
        origin: Vector {
            x: 0.,
            y: 0.,
            z: 1000.,
        },
        radius: 940.,
        material: opaque_white,
    };
    let front = Sphere {
        origin: Vector {
            x: 0.,
            y: 0.,
            z: -1000.,
        },
        radius: 940.,
        material: opaque_white,
    };
    let right = Sphere {
        origin: Vector {
            x: 1000.,
            y: 0.,
            z: 0.,
        },
        radius: 940.,
        material: opaque_blue,
    };
    let left = Sphere {
        origin: Vector {
            x: -1000.,
            y: 0.,
            z: 0.,
        },
        radius: 940.,
        material: opaque_red,
    };
    let bellow = Sphere {
        origin: Vector {
            x: 0.,
            y: 1000.,
            z: 0.,
        },
        radius: 990.,
        material: opaque_yellow,
    };
    let above = Sphere {
        origin: Vector {
            x: 0.,
            y: -1000.,
            z: 0.,
        },
        radius: 940.,
        material: opaque_green,
    };

    let mut scene = Scene::build(
        vec![
            Box::new(main),
            Box::new(back),
            Box::new(front),
            Box::new(right),
            Box::new(left),
            Box::new(bellow),
            Box::new(above),
        ],
        light,
        camera,
    );
    scene.generate_image();
}
