mod camera;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;

use glam::DVec3;

use sphere::Sphere;
use camera::Camera;
use hittable::HittableList;
// use rand::Rng;

fn main() {
    // Set up rng for later
    // let mut rng = rand::thread_rng();

    // Create a scene to add objects to
    let mut world = HittableList::default();

    // Ground
    world.add(Box::new(Sphere::new(
        DVec3::new(0.0, -100.5, -1.0),
        100.0,
        material::Material::Lambertian {
            albedo: DVec3::new(0.8, 0.8, 0.0),
        },
    )));

    // Dielectric Ball
    world.add(Box::new(Sphere::new(
        DVec3::new(1.0, 0.0, 0.0),
        0.5,
        material::Material::Dielectric {
            refraction_index: 1.5,
        },
    )));

    // // Dielectric Bubble
    // world.add(Box::new(Sphere::new(
    //     DVec3::new(1.0, 0.0, -1.0),
    //     0.4,
    //     material::Material::Dielectric {
    //         refraction_index: 1.0/1.5,
    //     },
    // )));

    // // Lambertian Ball
    // world.add(Box::new(Sphere::new(
    //     DVec3::new(1.2, 0.0, 0.0),
    //     0.5,
    //     material::Material::Lambertian {
    //         albedo: DVec3::new(0.1, 0.2, 0.5),
    //     },
    // )));

    // // Metal Ball
    // world.add(Box::new(Sphere::new(
    //     DVec3::new(1.0, 0.0, 1.0),
    //     0.5,
    //     material::Material::Metal {
    //         albedo: DVec3::new(0.8, 0.6, 0.2),
    //         fuzz: 1.0,
    //     },
    // )));

    // Lower Left Ball
    world.add(Box::new(Sphere::new(
        DVec3::new(1.5, -0.3, 0.75),
        0.2,
        material::Material::Lambertian {
            albedo: DVec3::new(0.0, 1.0, 1.0),
        },
    )));

    // // Lower Right Ball
    // world.add(Box::new(Sphere::new(
    //     DVec3::new(2.0, 0.4, 1.0),
    //     0.4,
    //     material::Material::Lambertian {
    //         albedo: DVec3::new(1.0, 1.0, 1.0),
    //     },
    // )));

    // for _ in 0..1 {
    //     let spread = 5.0;
    //     // Lambertian mini balls
    //     world.add(Box::new(Sphere::new(
    //         DVec3::new(
    //             (rng.gen::<f64>() - 0.5) * spread,
    //             0.15,
    //             (rng.gen::<f64>() - 0.5) * spread,
    //         ),
    //         0.15,
    //         material::Material::Lambertian {
    //             albedo: DVec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()),
    //         },
    //     )));

    //     // Metal mini balls
    //     world.add(Box::new(Sphere::new(
    //         DVec3::new(
    //             (rng.gen::<f64>() - 0.5) * spread,
    //             0.15,
    //             (rng.gen::<f64>() - 0.5) * spread,
    //         ),
    //         0.15,
    //         material::Material::Metal {
    //             albedo: DVec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()),
    //             fuzz: rng.gen::<f64>() / 2.0,
    //         },
    //     )));

    //     // Dielectric mini balls
    //     world.add(Box::new(Sphere::new(
    //         DVec3::new(
    //             (rng.gen::<f64>() - 0.5) * spread,
    //             0.15,
    //             (rng.gen::<f64>() - 0.5) * spread,
    //         ),
    //         0.15,
    //         material::Material::Dielectric {
    //             refraction_index: 1.0 / (rng.gen::<f64>() + 0.5),
    //         },
    //     )));
    // }

    let mut cam: Camera = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 200;
    cam.max_depth = 20;

    cam.vfov = 90.0;
    cam.look_from = DVec3::new(0.0, 0.0, 0.0);
    cam.look_at = DVec3::new(1.0, 0.0, 0.0);
    cam.look_up = DVec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 1.0;
    cam.focus_dist = 1.0;

    cam.render(&world);
}
