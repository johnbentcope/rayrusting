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
use ray::Ray;

fn main() {
    // Set up rng for later
    let mut rng = rand::thread_rng();

    // Create a scene to add objects to
    let mut world = HittableList::default();

    // Ground
    world.add(Box::new(Sphere::new(
        Ray::new(DVec3::new(0.0, -1000.0, -1.0), DVec3::ZERO, 0.0),
        1000.0,
        material::Material::Lambertian {
            albedo: DVec3::new(0.5, 0.5, 0.5),
        },
    )));

    // Dielectric Ball
    world.add(Box::new(Sphere::new(
        Ray::new(DVec3::new(0.0, 1.0, 0.0), DVec3::ZERO, 0.0),
        1.0,
        material::Material::Dielectric {
            refraction_index: 1.5,
        },
    )));

    // Lambertian Ball
    world.add(Box::new(Sphere::new(
        Ray::new(DVec3::new(-4.0, 1.0, 0.0), DVec3::ZERO, 0.0),
        1.0,
        material::Material::Lambertian {
            albedo: DVec3::new(0.4, 0.2, 0.1),
        },
    )));

    // Metal Ball
    world.add(Box::new(Sphere::new(
        Ray::new(DVec3::new(4.0, 1.0, 0.0), DVec3::ZERO, 0.0),
        1.0,
        material::Material::Metal {
            albedo: DVec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    )));

    for _ in 0..200 {
        let spread = 6.0;
        let vel = 1.0 / 3.0;

        let center = DVec3::new(
            (rng.gen::<f64>() - 0.5) * 2.0 * spread,
            0.2,
            (rng.gen::<f64>() - 0.5) * 2.0 * spread,
        );
        let speed = DVec3::new(0.0, rng.gen::<f64>() * vel, 0.0);

        // Lambertian mini balls
        world.add(Box::new(Sphere::new(
            Ray::new(center, speed, 0.0),
            0.15,
            material::Material::Lambertian {
                albedo: DVec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()),
            },
        )));

        let center = DVec3::new(
            (rng.gen::<f64>() - 0.5) * 2.0 * spread,
            0.2,
            (rng.gen::<f64>() - 0.5) * 2.0 * spread,
        );
        let speed = DVec3::new(0.0, rng.gen::<f64>() * vel, 0.0);

        // Lambertian mini balls
        world.add(Box::new(Sphere::new(
            Ray::new(center, speed, 0.0),
            0.15,
            material::Material::Metal {
                albedo: DVec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()),
                fuzz: rng.gen::<f64>() / 2.0,
            },
        )));

        let center = DVec3::new(
            (rng.gen::<f64>() - 0.5) * 2.0 * spread,
            0.2,
            (rng.gen::<f64>() - 0.5) * 2.0 * spread,
        );
        let speed = DVec3::new(0.0, rng.gen::<f64>() * vel, 0.0);

        // Lambertian mini balls
        world.add(Box::new(Sphere::new(
            Ray::new(center, speed, 0.0),
            0.15,
            material::Material::Dielectric {
                refraction_index: 1.0 / (rng.gen::<f64>() + 0.5),
            },
        )));
    }

    let mut cam: Camera = Camera::new();

    cam.aspect_ratio = 4.0 / 3.0;
    cam.image_width = 2560;
    cam.samples_per_pixel = 2000;
    cam.max_depth = 100;

    cam.vfov = 20.0;
    cam.look_from = DVec3::new(13.0, 2.0, 3.0);
    cam.look_at = DVec3::new(0.0, 0.0, 0.0);
    cam.look_up = DVec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}
