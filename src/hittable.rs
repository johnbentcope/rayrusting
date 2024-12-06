use crate::aabb::AABB;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use glam::DVec3;

#[derive(Debug, Clone, Copy, Default)]
pub struct HitRecord {
    pub p: DVec3,         // point of hit
    pub normal: DVec3,    // normal vector at point of hit
    pub t: f64,           // distance traveled to hit
    pub front_face: bool, // did ray intersect the front face of the object
    pub mat: Material,    // what material was hit
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: DVec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, ray_t: Interval, debug: bool) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AABB>;
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.bbox = AABB::from_boxes(&self.bbox, &obj.bounding_box().unwrap());
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    // Casts a Ray into a scene, and returns the closest HitRecord
    fn hit(&self, r: &Ray, ray_t: Interval, debug: bool) -> Option<HitRecord> {
        // Set the water line to the max distance
        let mut closest_so_far = ray_t.max;
        if debug {
            println!("hittable::hit::closest_so_far::{:?}", closest_so_far);
            println!("hittable::hit::ray_t::{:?}", ray_t);
        }
        // Initialize a hit_record to return
        let mut hit_record: Option<HitRecord> = None;

        // Loop through all objects in the scene
        for object in self.objects.iter() {
            // If an object returns that there is a valid hit between the minimum distance and the water line
            if let Some(temp_rec) = object.hit(r, Interval::new(ray_t.min, closest_so_far), debug) {
                // Update the water line
                closest_so_far = temp_rec.t;
                if debug {
                    println!("hittable::hit::closest_so_far(loop)::{:?}", closest_so_far);
                }

                // Update hit_record with the HitRecord of the hit
                hit_record = Some(temp_rec);
            }
        }
        hit_record
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bbox)
    }
}
