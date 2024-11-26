mod camera;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;

use glam::DVec3;

use camera::Camera;
use hittable::HittableList;
use rand::Rng;
use sphere::Sphere;

fn main() {
    let mut world = HittableList::default();

    // Ground
    world.add(Box::new(Sphere::new(
        DVec3::new(0.0, -1000.51, -1.5),
        1000.0,
        material::Material::Lambertian {
            albedo: DVec3::new(0.8, 0.8, 0.0),
        },
    )));

    // Left Ball
    world.add(Box::new(Sphere::new(
        DVec3::new(-1.0, 0.0, -1.5),
        0.5,
        material::Material::Dielectric {
            refraction_index: 1.5,
        },
    )));

    // Left Ball Air Bubble
    world.add(Box::new(Sphere::new(
        DVec3::new(-1.0, 0.0, -1.5),
        0.4,
        material::Material::Dielectric {
            refraction_index: 1.0 / 1.5,
        },
    )));

    // Middle Ball
    world.add(Box::new(Sphere::new(
        DVec3::new(0.0, 0.0, -1.5),
        0.5,
        material::Material::Lambertian {
            albedo: DVec3::new(0.1, 0.2, 0.5),
        },
    )));

    // Right Ball
    world.add(Box::new(Sphere::new(
        DVec3::new(1.0, 0.0, -1.5),
        0.5,
        material::Material::Metal {
            albedo: DVec3::new(0.9, 0.69, 0.15),
            fuzz: 0.2,
        },
    )));

    // Lower Left Ball
    world.add(Box::new(Sphere::new(
        DVec3::new(-0.5, -0.35, -1.2),
        0.15,
        material::Material::Metal {
            albedo: DVec3::new(1.0, 1.0, 1.0),
            fuzz: 0.0,
        },
    )));

    // Lower Right Ball
    world.add(Box::new(Sphere::new(
        DVec3::new(0.5, -0.35, -1.2),
        0.15,
        material::Material::Lambertian {
            albedo: DVec3::new(1.0, 1.0, 1.0),
        },
    )));
    let mut rng = rand::thread_rng();

    for _ in 0..50 {
        // Lambertian mini balls
        world.add(Box::new(Sphere::new(
            DVec3::new(
                (rng.gen::<f64>() - 0.5) * 3.5,
                0.25 * (rng.gen::<f64>()) - 0.45,
                (rng.gen::<f64>() - 0.5) * 1.0,
            ),
            0.05,
            material::Material::Lambertian {
                albedo: DVec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()),
            },
        )));
    }
    for _ in 0..50 {
        // Metallic mini balls
        world.add(Box::new(Sphere::new(
            DVec3::new(
                (rng.gen::<f64>() - 0.5) * 4.0,
                0.25 * (rng.gen::<f64>()) - 0.45,
                (rng.gen::<f64>() - 0.5) * 1.0,
            ),
            0.05,
            material::Material::Metal {
                albedo: DVec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()),
                fuzz: rng.gen::<f64>() / 2.0,
            },
        )));
    }
    for _ in 0..50 {
        // Dielectric mini balls
        world.add(Box::new(Sphere::new(
            DVec3::new(
                (rng.gen::<f64>() - 0.5) * 3.0,
                0.25 * (rng.gen::<f64>()) - 0.45,
                (rng.gen::<f64>() - 0.5) * 1.0,
            ),
            0.05,
            material::Material::Dielectric {
                refraction_index: 1.0 / (rng.gen::<f64>() + 0.5),
            },
        )));
    }

    let mut cam: Camera = Camera::new();

    cam.aspect_ratio = 4.0 / 3.0;
    cam.image_width = 800;
    cam.samples_per_pixel = 1500;

    cam.vfov = 40.0;
    cam.look_from = DVec3::new(-1.0, 0.5, 2.0);
    cam.look_at = DVec3::new(-0.125, -0.4, -1.5);
    cam.look_up = DVec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 3.0;
    cam.focus_dist = 3.4;

    cam.render(&world);
}
