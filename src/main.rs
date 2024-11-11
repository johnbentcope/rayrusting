mod ray;
mod hittable;
mod sphere;
mod interval;
mod camera;
mod utils;

use glam::{Vec3, DVec3};

use hittable::HittableList;
use sphere::Sphere;
use camera::Camera;

fn main() {
    let mut world = HittableList::default();

    world.add(Box::new(Sphere::new(DVec3::new(0.0,0.0,-1.0), 0.5)));
    world.add(Box::new(Sphere::new(DVec3::new(0.0,-100.5,-1.0), 100.0)));

    let mut cam:Camera = Camera::new();

    cam.aspect_ratio = 4.0 / 3.0;
    cam.image_width = 320;
    cam.samples_per_pixel = 50;

    cam.render(&world);
}