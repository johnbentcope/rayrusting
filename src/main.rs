mod ray;
mod hittable;
mod sphere;

use std::f32::INFINITY;
use std::fs::File;
use std::io::{BufWriter, Write};
use glam::Vec3;

use hittable::HittableList;
use sphere::Sphere;
use crate::hittable::Hittable;
use ray::Ray;

fn main() {
    // Debug flag for verbosity
    let debug = false;

    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0)));

    // File I/O Setup
    let file = File::create("image.ppm").expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 600;
    
    // Calculate the image height, and ensure that it's at least 1.
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ( (image_width as f32)/(image_height as f32) );
    let camera_center = Vec3::new(0.0,0.0,0.0);

    // Create the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Create the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    write_header(&mut writer, &image_width, &image_height);

    for row in 0..image_height {
        if debug == true {
            let rem = image_height-1;
            println!("Writing scanline {row} of {rem}");
        }
        for col in 0..image_width {

            let pixel_center = pixel00_loc + (col as f32 * pixel_delta_u) + (row as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let color = ray_color(& r, &world);

            write_color(&mut writer, &color).unwrap();

        }
    }
}


fn ray_color(r: & Ray, world: &HittableList) -> Vec3 {
    let blue = Vec3::new(0.5, 0.7, 1.0);
    let white = Vec3::new(1.0, 1.0, 1.0);
    if let Some(rec) = world.hit(r,0.0,INFINITY) {
        return 0.5 * (rec.normal + white);
    }

    let unit_direction = r.direction.normalize();
    let a = 0.5*(unit_direction.y + 1.0);
    return white.lerp(blue, a);
}

fn write_header(writer: &mut BufWriter<File>, width: &i32, height: &i32) -> () {
    writeln!(writer, "P3").unwrap();
    writeln!(writer, "{width} {height}").unwrap();
    writeln!(writer, "255").unwrap();
}

fn write_color(writer: &mut BufWriter<File>, color: &Vec3) -> Result<(), std::io::Error> {
    let r: i32 = (color.x * 255.99) as i32;
    let g: i32 = (color.y * 255.99) as i32;
    let b: i32 = (color.z * 255.99) as i32;

    writeln!(writer, "{r:>3} {g:>3} {b:>3}")
}