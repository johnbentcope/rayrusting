use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use glam::DVec3;

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Material,
}

impl Sphere {
    pub fn new(center: Ray, radius: f64, mat: Material) -> Sphere {
        Sphere {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, debug: bool) -> Option<HitRecord> {
        let current_center = self.center.at(r.time);
        let oc = current_center - r.origin;
        let a = r.direction.dot(r.direction);
        let b = -2.0 * r.direction.dot(oc);
        let c = oc.dot(oc) - (self.radius * self.radius);

        let discriminant = (b * b) - (4.0 * a * c);

        if debug {
            println!("sphere::hit::r: {:?}", r);
            println!("sphere::hit::self.center: {:?}", self.center);
            println!("sphere::hit::self.radius: {:?}", self.radius);
            println!("sphere::hit::a: {:?}", a);
            println!("sphere::hit::b: {:?}", b);
            println!("sphere::hit::c: {:?}", c);
            println!("sphere::hit::r.time: {:?}", r.time);
            println!("sphere::hit::current_center: {:?}", current_center);
        }

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        if debug {
            println!("sphere::hit::sqrt_d: {:?}", sqrt_d);
        }

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-b - sqrt_d) / (2.0 * a);

        if !ray_t.surrounds(root) {
            root = (-b + sqrt_d) / (2.0 * a);
            if debug {
                println!("not first {}", root)
            }
            if !ray_t.surrounds(root) {
                if debug {
                    println!("not either {}", root)
                }
                return None;
            } else {
                if debug {
                    println!("was second with {}", root)
                }
            }
        } else {
            if debug {
                println!("was first {}", root)
            }
        }
        if debug {
            println!("sphere::hit::root: {:?}", root);
        }

        let mut rec = HitRecord {
            p: r.at(root),
            t: root,
            normal: DVec3::ZERO,
            front_face: false,
            mat: self.mat,
        };

        let outward_normal = (rec.p - current_center) / self.radius;

        rec.set_face_normal(r, outward_normal);

        if debug {
            println!("sphere::hit::rec: {:?}", rec);
        }
        Some(rec)
    }
}

// TODO test hit function
