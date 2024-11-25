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
use sphere::Sphere;

fn main() {
    let mut world = HittableList::default();

    // Ground
    world.add(Box::new(Sphere::new(
        DVec3::new(0.0, -100.6, -1.5),
        100.0,
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
            fuzz: 1.0,
        },
    )));

    // Lower Left Ball
    world.add(Box::new(Sphere::new(
        DVec3::new(-0.5, -0.42, -1.0),
        0.15,
        material::Material::Metal {
            albedo: DVec3::new(1.0, 1.0, 1.0),
            fuzz: 0.0,
        },
    )));

    // Lower Right Ball
    world.add(Box::new(Sphere::new(
        DVec3::new(0.5, -0.42, -1.0),
        0.15,
        material::Material::Lambertian {
            albedo: DVec3::new(1.0, 1.0, 1.0),
        },
    )));

    let mut cam: Camera = Camera::new();

    cam.aspect_ratio = 4.0 / 3.0;
    cam.image_width = 640;
    cam.samples_per_pixel = 10;
    cam.vfov = 40.0;
    cam.look_from = DVec3::new(-2.0, 2.0, 1.0);
    cam.look_at = DVec3::new(0.0, 0.0, -1.25);
    cam.look_up = DVec3::new(0.0, 1.0, 0.0);

    cam.render(&world);
}
