use rand::Rng;

pub const INFINITY: f64 = f64::MAX;
pub const PI: f64 = 3.1415926535897932385;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

#[inline]
pub fn random_double() -> f64 {
    return rand::random::<f64>();
}

#[inline]
pub fn random_rng(min: f64, max: f64) -> f64 {
    return min + ((max - min) * rand::random::<f64>());
}