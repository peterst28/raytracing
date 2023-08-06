use crate::vec3::Vec3;
use std::ops;
use std::ops::Neg;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug)]
pub struct Color {
    vec: Vec3,
}

impl Color {
    pub fn new_default() -> Color {
        Color { vec: Vec3::new_default() }
    }

    pub fn new(e0: f64, e1: f64, e2: f64) -> Color {
        Color { vec: Vec3::new(e0, e1, e2) }
    }

    pub fn x(&self) -> f64 {
        self.vec.x()
    }

    pub fn y(&self) -> f64 {
        self.vec.y()
    }

    pub fn z(&self) -> f64 {
        self.vec.z()
    }

    pub fn length(&self) -> f64 {
        self.vec.length()
    }

    pub fn length_squared(&self) -> f64 {
        self.vec.length_squared()
    }
}

impl ops::AddAssign for Color {

    fn add_assign(&mut self, other: Self) {
        self.vec += other.vec
    }
}

impl ops::MulAssign<f64> for Color {

    fn mul_assign(&mut self, rhs: f64) {
        self.vec *= rhs
    }
}

impl ops::DivAssign<f64> for Color {

    fn div_assign(&mut self, rhs: f64) {
        self.vec /= rhs
    }
}

impl Neg for Color {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            vec: -self.vec
        }
    }
}

impl Index<usize> for Color {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.vec[i]
    }
}

impl IndexMut<usize> for Color {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.vec[i]
    }
}