use crate::interval::Interval;
use crate::ray::Ray;
use glam::DVec3;

#[derive(Default, Debug, Clone, Copy)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    // The default AABB is empty, since intervals are empty by default.
    pub fn new() -> AABB {
        AABB {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    // The intuitive constructor, explicitly providing all intervals.
    pub fn from_intervals(x: Interval, y: Interval, z: Interval) -> AABB {
        AABB { x, y, z }
    }

    // Treat the two points a and b as extrema for the bounding box, so we don't require a
    // particular minimum/maximum coordinate order.
    pub fn from_points(a: DVec3, b: DVec3) -> AABB {
        AABB {
            x: Interval::new(a.x.min(b.x), a.x.max(b.x)),
            y: Interval::new(a.y.min(b.y), a.y.max(b.y)),
            z: Interval::new(a.z.min(b.z), a.z.max(b.z)),
        }
    }
    // Treat the two points a and b as extrema for the bounding box, so we don't require a
    // particular minimum/maximum coordinate order.
    pub fn from_boxes(box0: &AABB, box1: &AABB) -> AABB {
        AABB {
            x: Interval::from_intervals(box0.x, box1.x),
            y: Interval::from_intervals(box0.y, box1.y),
            z: Interval::from_intervals(box0.z, box1.z),
        }
    }

    pub fn axis_interval(&self, axis: i32) -> Interval {
        match axis {
            0 => self.x,
            1 => self.y,
            _ => self.z,
        }
    }

    pub fn hit(&self, r: Ray, ray_t: Interval) -> bool {
        let mut ray_t = ray_t;
        for axis in 0..=2 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / r.direction[axis as usize]; // adinv is axis direction inverse

            let t0 = (ax.min - r.origin[axis as usize]) * adinv;
            let t1 = (ax.max - r.origin[axis as usize]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }
}
