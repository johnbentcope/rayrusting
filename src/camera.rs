// use crate::hittable::Hittable;
use glam::Vec3;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::hittable::Hittable;
use crate::hittable::HittableList;
use std::fs::File;
use rand::Rng;
use std::io::{BufWriter, Write};
use std::f32::{INFINITY, NEG_INFINITY};


pub struct Camera{
    pub aspect_ratio: f32,
    pub image_width: i32,
    image_height: i32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: i32,
    pixel_samples_scale : f32,
}

impl Camera{
    pub fn new() -> Camera{
        Camera {
            aspect_ratio: 1.0,
            image_width: 600,
            image_height: 600,
            center: Vec3::new(0.0,0.0,0.0),
            pixel00_loc: Vec3::new(0.0,0.0,0.0),
            pixel_delta_u: Vec3::new(0.0,0.0,0.0),
            pixel_delta_v: Vec3::new(0.0,0.0,0.0),
            samples_per_pixel: 30,
            pixel_samples_scale : 0.0,
        }
    }

    pub fn render(&mut self, world: &HittableList){
        // Debug flag for verbosity
        let debug = false;

        self.initialize();

        // File I/O Setup
        let file = File::create("image.ppm").expect("Failed to create file");
        let mut writer = BufWriter::new(file);

        Self::write_header(&mut writer, &self.image_width, &self.image_height);

        for row in 0..self.image_height {
            if debug == true {
                let rem = self.image_height-1;
                println!("Writing scanline {row} of {rem}");
            }
            for col in 0..self.image_width {

                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0); 
                
                for i in (0..self.samples_per_pixel) {
                    let r = Self::get_ray(self, col, row);
                    pixel_color += Self::ray_color(&r, world);
                }

                let pixel_color = self.pixel_samples_scale * pixel_color;
                Self::write_color(&mut writer, &pixel_color).unwrap();

            }
        }
    }

    fn ray_color(r: & Ray, world: &HittableList) -> Vec3 {
        let blue = Vec3::new(0.5, 0.7, 1.0);
        let white = Vec3::new(1.0, 1.0, 1.0);
        if let Some(rec) = world.hit(r,Interval::new(0.0,INFINITY)) {
            return 0.5 * (rec.normal + white);
        }

        let unit_direction = r.direction.normalize();
        let a = 0.5*(unit_direction.y + 1.0);
        return white.lerp(blue, a);
    }

    fn get_ray(&self, x: i32, y: i32) -> Ray{
        
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc + ((x as f32 + offset.x) * self.pixel_delta_u) + ((y as f32 + offset.y) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        return Ray::new(ray_origin, ray_direction);
    }

    fn sample_square() -> Vec3 {
        let mut rng = rand::thread_rng();

        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        return Vec3::new(rng.gen::<f32>() - 0.5, rng.gen::<f32>() - 0.5, 0.0);
    }

    fn initialize(&mut self) {
        // Calculate the image height, and ensure that it's at least 1.
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };
        self.center = Vec3::new(0.0,0.0,0.0);

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f32;

        // Initialize viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * ( (self.image_width as f32)/(self.image_height as f32) );

        // Create the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Create the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn write_header(writer: &mut BufWriter<File>, width: &i32, height: &i32) -> () {
        writeln!(writer, "P3").unwrap();
        writeln!(writer, "{width} {height}").unwrap();
        writeln!(writer, "255").unwrap();
    }
    
    fn write_color(writer: &mut BufWriter<File>, color: &Vec3) -> Result<(), std::io::Error> {
        let r  = color.x;
        let g  = color.y;
        let b  = color.z;

        let intensity = Interval::new(0.000, 0.999);
        let r: i32 = (intensity.clamp(r) * 256.) as i32;
        let g: i32 = (intensity.clamp(g) * 256.) as i32;
        let b: i32 = (intensity.clamp(b) * 256.) as i32;
    
        writeln!(writer, "{r:>3} {g:>3} {b:>3}")
    }

}
