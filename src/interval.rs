use std::f64::{INFINITY, NEG_INFINITY};

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn _size(&self) -> f64 {
        self.max - self.min
    }
    pub fn _contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}

pub const _EMPTY: Interval = Interval {
    min: INFINITY,
    max: NEG_INFINITY,
};
pub const _UNIVERSE: Interval = Interval {
    min: NEG_INFINITY,
    max: INFINITY,
};
