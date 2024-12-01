mod camera;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;

use glam::DVec3;

use crate::sphere::Sphere;
use camera::Camera;
use hittable::HittableList;
use rand::Rng;

fn main() {
    // Set up rng for later
    let mut rng = rand::thread_rng();

    // Create a scene to add objects to
    let mut world = HittableList::default();

    // Ground
    world.add(Box::new(Sphere::new(
        DVec3::new(0.0, -1000.0, -1.0),
        1000.0,
        material::Material::Lambertian {
            albedo: DVec3::new(0.5, 0.5, 0.5),
        },
    )));

    // Dielectric Ball
    world.add(Box::new(Sphere::new(
        DVec3::new(0.0, 1.0, 0.0),
        1.0,
        material::Material::Dielectric {
            refraction_index: 1.5,
        },
    )));

    // Lambertian Ball
    world.add(Box::new(Sphere::new(
        DVec3::new(-4.0, 1.0, -0.0),
        1.0,
        material::Material::Lambertian {
            albedo: DVec3::new(0.4, 0.2, 0.1),
        },
    )));

    // Metal Ball
    world.add(Box::new(Sphere::new(
        DVec3::new(4.0, 1.0, 0.0),
        1.0,
        material::Material::Metal {
            albedo: DVec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    )));

    // Lower Left Ball
    world.add(Box::new(Sphere::new(
        DVec3::new(-2.0, 0.4, 1.0),
        0.4,
        material::Material::Metal {
            albedo: DVec3::new(1.0, 1.0, 1.0),
            fuzz: 0.0,
        },
    )));

    // Lower Right Ball
    world.add(Box::new(Sphere::new(
        DVec3::new(2.0, 0.4, 1.0),
        0.4,
        material::Material::Lambertian {
            albedo: DVec3::new(1.0, 1.0, 1.0),
        },
    )));

    for _ in 0..50 {
        let spread = 20.0;
        // Lambertian mini balls
        world.add(Box::new(Sphere::new(
            DVec3::new(
                (rng.gen::<f64>() - 0.5) * spread,
                0.15,
                (rng.gen::<f64>() - 0.5) * spread,
            ),
            0.15,
            material::Material::Lambertian {
                albedo: DVec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()),
            },
        )));

        // Lambertian mini balls
        world.add(Box::new(Sphere::new(
            DVec3::new(
                (rng.gen::<f64>() - 0.5) * spread,
                0.15,
                (rng.gen::<f64>() - 0.5) * spread,
            ),
            0.15,
            material::Material::Metal {
                albedo: DVec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()),
                fuzz: rng.gen::<f64>() / 2.0,
            },
        )));

        // Lambertian mini balls
        world.add(Box::new(Sphere::new(
            DVec3::new(
                (rng.gen::<f64>() - 0.5) * spread,
                0.15,
                (rng.gen::<f64>() - 0.5) * spread,
            ),
            0.15,
            material::Material::Dielectric {
                refraction_index: 1.0 / (rng.gen::<f64>() + 0.5),
            },
        )));
    }

    let mut cam: Camera = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 2560;
    cam.samples_per_pixel = 1500;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.look_from = DVec3::new(13.0, 2.0, 3.0);
    cam.look_at = DVec3::new(0.0, 1.25, 0.0);
    cam.look_up = DVec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}
