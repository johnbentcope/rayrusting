use glam::{Vec3, DVec3};
use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::interval::Interval;

pub struct Sphere {
    center: DVec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let h = r.direction.dot(oc);
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = h*h - a*c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let root = (h - sqrt_d) / a;
        if !ray_t.surrounds(root) {
            let root = (h + sqrt_d) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }
        
        let mut rec = HitRecord {
            p: r.at(root),
            t: root,
            normal: DVec3::ZERO,
            front_face: false,
        };

        let outward_normal = (rec.p - self.center) / self.radius;
        
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}
