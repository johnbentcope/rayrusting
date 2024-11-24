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
    // let material_left   = Metal::new(color(0.8, 0.8, 0.8));
    // let material_right  = Metal::new(color(0.8, 0.6, 0.2));

    world.add(Box::new(Sphere::new(
        DVec3::new(0.0, -100.6, -1.5),
        100.0,
        material::Material::Lambertian{albedo: DVec3::new(1.0, 1.0, 1.0)},
    )));
    world.add(Box::new(Sphere::new(
        DVec3::new(-1.0, 0.0, -1.5),
        0.5,
        material::Material::Metal {albedo: DVec3::new(0.8, 0.1, 0.1), fuzz: 0.05},
    )));
    world.add(Box::new(Sphere::new(
        DVec3::new(0.0, 0.0, -1.5),
        0.5,
        material::Material::Metal {albedo: DVec3::new(0.1, 0.8, 0.1), fuzz: 0.0000001},
    )));
    world.add(Box::new(Sphere::new(
        DVec3::new(1.0, 0.0, -1.5),
        0.5,
        material::Material::Lambertian {albedo: DVec3::new(0.1, 0.1, 0.8)},
    )));
    world.add(Box::new(Sphere::new(
        DVec3::new(-0.5, -0.42, -1.5),
        0.15,
        material::Material::Metal {albedo: DVec3::new(1.0, 1.0, 1.0), fuzz: 0.001},
    )));
    world.add(Box::new(Sphere::new(
        DVec3::new(0.5, -0.42, -1.5),
        0.15,
        material::Material::Lambertian {albedo: DVec3::new(1.0, 1.0, 1.0)},
    )));

    let mut cam: Camera = Camera::new();

    cam.aspect_ratio = 4.0 / 2.4;
    cam.image_width = 1440;
    cam.samples_per_pixel = 1000;

    cam.render(&world);
}
