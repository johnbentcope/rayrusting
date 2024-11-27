// use crate::hittable::Hittable;
use crate::hittable::Hittable;
use crate::hittable::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utils::random_in_unit_disc;
use glam::DVec3;
use rand::Rng;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: i32,
    image_height: i32,
    center: DVec3,
    pixel00_loc: DVec3,
    pixel_delta_u: DVec3,
    pixel_delta_v: DVec3,
    pub samples_per_pixel: i32,
    pixel_samples_scale: f64,
    max_depth: i32,
    pub vfov: f64,
    pub look_from: DVec3,
    pub look_at: DVec3,
    pub look_up: DVec3,

    pub defocus_angle: f64,
    pub focus_dist: f64,
    defocus_disc_u: DVec3,
    defocus_disc_v: DVec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            aspect_ratio: 1.0,
            image_width: 600,
            image_height: 600,
            center: DVec3::new(0.0, 0.0, 0.0),
            pixel00_loc: DVec3::new(0.0, 0.0, 0.0),
            pixel_delta_u: DVec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: DVec3::new(0.0, 0.0, 0.0),
            samples_per_pixel: 10,
            pixel_samples_scale: 0.0,
            max_depth: 10,
            vfov: 90.0,
            look_from: DVec3::new(0.0, 0.0, 0.0),
            look_at: DVec3::new(0.0, 0.0, -1.0),
            look_up: DVec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            defocus_disc_u: DVec3::new(0.0, 1.0, 0.0),
            defocus_disc_v: DVec3::new(0.0, 1.0, 0.0),
        }
    }

    fn initialize(&mut self) {
        // Calculate the image height, and ensure that it's at least 1.
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        // Scaling factor to divide a pixel's color by when summating all contributing samples
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.look_from;

        // Initialize viewport dimensions
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width =
            viewport_height * ((self.image_width as f64) / (self.image_height as f64));

        // i unit vector from camera's frame of reference
        let w = (self.look_from - self.look_at).normalize();
        // j unit vector from camera's frame of reference
        let u = self.look_up.cross(w);
        // k unit vector from camera's frame of reference
        let v = w.cross(u);

        // Create the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Create the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - (self.focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disc_u = u * defocus_radius;
        self.defocus_disc_v = v * defocus_radius;
    }

    pub fn render(&mut self, world: &HittableList) {
        // Debug flag for verbosity
        let debug = true;

        self.initialize();

        // File I/O Setup
        let file = File::create("image.ppm").expect("Failed to create file");
        let mut writer = BufWriter::new(file);

        Self::write_header(&mut writer, &self.image_width, &self.image_height);

        for row in 0..self.image_height {
            if debug {
                let rem = self.image_height - 1;
                println!("Writing scanline {row} of {rem}");
            }

            // Create a buffer to store pixels and coordinates in for the asynchronous rendering
            let row_buffer: Arc<Mutex<Vec<(usize, DVec3)>>> = Arc::new(Mutex::new(Vec::new()));
            (0..self.image_width).into_par_iter().for_each(|col| {
                let mut pixel_color = DVec3::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let r = Self::get_ray(self, col, row);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }

                let pixel_color = self.pixel_samples_scale * pixel_color;
                let mut buffer = row_buffer.lock().unwrap();
                buffer.push((col.try_into().unwrap(), pixel_color));
            });

            // Once the row is fully drawn by each thread, get the row_buffer and allow mutation
            let mut row_buffer = Arc::try_unwrap(row_buffer).unwrap().into_inner().unwrap();

            // sort the row_buffer by column position
            row_buffer.sort_by_key(|&(col, _)| col);

            // Print it
            for (_, pixel_color) in row_buffer {
                Self::write_color(&mut writer, &pixel_color).unwrap();
            }
        }
    }

    fn ray_color(r: &Ray, depth: i32, world: &HittableList) -> DVec3 {
        if depth <= 0 {
            return DVec3::new(1.0, 0.0, 1.0);
        }
        let blue = DVec3::new(0.5, 0.7, 1.0);
        let white = DVec3::new(1.0, 1.0, 1.0);

        if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            let (attenuation, scattered, keeps_bouncing) = rec.mat.scatter(*r, &rec).unwrap();
            if keeps_bouncing {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            } else {
                return DVec3::new(0.0, 0.0, 0.0);
            }
        }

        let unit_direction = r.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        white.lerp(blue, a)
    }

    fn get_ray(&self, x: i32, y: i32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((x as f64 + offset.x) * self.pixel_delta_u)
            + ((y as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            Self::defocus_disc_sample(self)
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disc_sample(&self) -> DVec3 {
        let p = random_in_unit_disc();
        self.center + (p[0] * self.defocus_disc_u) + (p[1] * self.defocus_disc_v)
    }

    fn sample_square() -> DVec3 {
        let mut rng = rand::thread_rng();

        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        DVec3::new(rng.gen::<f64>() - 0.5, rng.gen::<f64>() - 0.5, 0.0)
    }

    fn write_header(writer: &mut BufWriter<File>, width: &i32, height: &i32) {
        writeln!(writer, "P3").unwrap();
        writeln!(writer, "{width} {height}").unwrap();
        writeln!(writer, "255").unwrap();
    }

    fn write_color(writer: &mut BufWriter<File>, color: &DVec3) -> Result<(), std::io::Error> {
        let r = color.x;
        let g = color.y;
        let b = color.z;

        let r = Self::linear_to_gamma(r);
        let g = Self::linear_to_gamma(g);
        let b = Self::linear_to_gamma(b);

        let intensity = Interval::new(0.000, 0.999);
        let r: i32 = (intensity.clamp(r) * 256.) as i32;
        let g: i32 = (intensity.clamp(g) * 256.) as i32;
        let b: i32 = (intensity.clamp(b) * 256.) as i32;

        writeln!(writer, "{r:>3} {g:>3} {b:>3}")
    }

    fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            return linear_component.sqrt();
        }
        0.0
    }
}
