use glam::DVec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: DVec3) -> Ray {
        Ray {
            origin,
            direction: DVec3::ZERO,
            time: 0.0,
        }
    }

    pub fn with_direction(origin: DVec3, direction: DVec3) -> Ray {
        Ray {
            origin,
            direction,
            time: 0.0,
        }
    }

    pub fn with_time(origin: DVec3, direction: DVec3, time: f64) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + self.direction * t
    }
}
