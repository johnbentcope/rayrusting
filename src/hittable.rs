use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use glam::DVec3;

// #[derive(Debug, Clone, Copy, Default)]
pub struct HitRecord {
    pub p: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Material,
}

impl HitRecord {
    pub fn _new(p: DVec3, normal: DVec3, t: f64, front_face: bool, mat: Material) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            front_face,
            mat,
        }
    }
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
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn _new(object: Box<dyn Hittable>) -> Self {
        {
            let mut list = Self::default();
            list.objects.push(object);
            list
        }
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }

    pub fn _clear(mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut hit_record: Option<HitRecord> = None;

        for object in self.objects.iter() {
            if let Some(temp_rec) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = temp_rec.t;
                hit_record = Some(temp_rec);
            }
        }
        hit_record
    }
}
