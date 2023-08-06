use std::ops;
use std::ops::Neg;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug)]
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

impl ops::DivAssign<f64> for Vec3 {

    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0/rhs
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