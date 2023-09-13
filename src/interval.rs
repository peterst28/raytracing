use crate::rtweekend;

pub struct Interval {
    pub max: f64,
    pub min: f64
}

impl Interval {
    pub fn new_default() -> Interval {
        Interval {
            min: rtweekend::INFINITY,
            max: -rtweekend::INFINITY
        }
    }

    pub fn new(min: f64, max: f64) -> Interval {
        Interval {
            min: min,
            max: max
        }
    }

    pub fn contains(&self, x:f64) -> bool {
        return self.min <= x && x <= self.max;
    }

    pub fn surrounds(&self, x:f64) -> bool {
        return self.min < x && x < self.max;
    }

    pub fn clamp(&self, x:f64) -> f64 {
        if x < self.min {
            return self.min
        }
        if x > self.max {
            return self.max
        }
        return x;
    }
}


pub const EMPTY: Interval = Interval {
    min: rtweekend::INFINITY,
    max: -rtweekend::INFINITY
};
pub const UNIVERSE: Interval = Interval {
    min: -rtweekend::INFINITY, 
    max: rtweekend::INFINITY
};