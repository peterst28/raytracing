use std::ops;
use std::ops::Neg;
use std::ops::Index;
use std::ops::IndexMut;
use crate::interval::Interval;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub e: [f64;3],
}

impl Vec3 {
    pub fn new_default() -> Vec3 {
        Vec3 { e: [0.0,0.0,0.0] }
    }

    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        return f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        return self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn dot(&self, v: &Vec3) -> f64 {
          self.e[0] * v.e[0]
        + self.e[1] * v.e[1]
        + self.e[2] * v.e[2]
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * v.e[2] - self.e[2] * v.e[1],
                self.e[2] * v.e[0] - self.e[0] * v.e[2],
                self.e[0] * v.e[1] - self.e[1] * v.e[0]
            ]
        }
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        return *v / v.length()
    }

    pub fn write_color(&self, samples_per_pixel: i32) -> String {

        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        let scale = 1.0 / (samples_per_pixel as f64);
        r *= scale;
        g *= scale;
        b *= scale;

        format!("{} {} {}",
            (256.0 * Vec3::INTENSITY.clamp(r)) as u32,
            (256.0 * Vec3::INTENSITY.clamp(g)) as u32,
            (256.0 * Vec3::INTENSITY.clamp(b)) as u32
        )

        // return format!("{} {} {}",
        //     (256.0 * self.x()) as u32,
        //     (256.0 * self.y()) as u32,
        //     (256.0 * self.z()) as u32
        // );
    }

    const INTENSITY: Interval = Interval {
        min: 0.0, 
        max: 0.999
    };

}

impl ops::AddAssign for Vec3 {

    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ]
        }
    }
}

impl ops::Add for Vec3 {

    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ]
        }
    }
}

impl ops::Sub for Vec3 {

    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ]
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {

    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            e: [
                self.e[0] * rhs,
                self.e[1] * rhs,
                self.e[2] * rhs,
            ]
        }
    }
}

impl ops::Mul<f64> for Vec3 {

    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            e: [
                self.e[0] * rhs,
                self.e[1] * rhs,
                self.e[2] * rhs,
            ]
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {

    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ]
        }
    }
}

impl ops::Mul<Vec3> for f64 {

    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl ops::DivAssign<f64> for Vec3 {

    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0/rhs
    }
}

impl ops::Div<f64> for Vec3 {

    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        (1.0 / rhs) * self
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            e: [
                -self.e[0],
                -self.e[1],
                -self.e[2],
            ]
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.e[i]
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

pub type Color = Vec3;
pub type Point3 = Vec3;