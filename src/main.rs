use std::ops;
use std::ops::Neg;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug)]
struct Vec3 {
    e: [f64;3],
}

trait Point3 {}
trait Color {}

impl Color for Vec3 {}
impl Point3 for Vec3 {}

impl Vec3 {
    fn new_default() -> Vec3 {
        Vec3 { e: [0.0,0.0,0.0] }
    }

    fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    fn x(&self) -> f64 {
        self.e[0]
    }

    fn y(&self) -> f64 {
        self.e[1]
    }

    fn z(&self) -> f64 {
        self.e[2]
    }

    fn length(&self) -> f64 {
        return f64::sqrt(self.length_squared())
    }

    fn length_squared(&self) -> f64 {
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

fn main() {
    let mut pt1 = Vec3::new(1.0,2.0,3.0);
    let pt2: Vec3 = Vec3::new(5.0, 6.0, 7.0);
    pt1 += pt2;
    println!("{:?}", pt1);
    pt1 *= 2.0;
    pt1 = -pt1;
    println!("{:?}", pt1);
    let x = pt1[0];
    println!("pt1[0]: {}", x);
    pt1[0] = 10.0;
    println!("{:?}", pt1);
    pt1 /= 10.0;
    println!("{:?}", pt1);

    let mut c:Vec3 = Vec3::new_default();
    // let image_height: u32 = 256;
    // let image_width: u32 = 256;

    // print!("P3\n{} {}\n255\n", image_width, image_height);

    // for j in (0..image_height-1).rev() {
    //     eprintln!("Scanlines remaining: {}", j);
    //     for i in 0..image_width {
    //         let r: f64 = i as f64 / (image_width-1) as f64;
    //         let g: f64 = j as f64 / (image_height-1) as f64;
    //         let b: f64 = 0.25;

    //         let ir: u32 = (255.999 * r) as u32;
    //         let ig: u32 = (255.999 * g) as u32;
    //         let ib: u32 = (255.999 * b) as u32;

    //         println!("{} {} {}", ir, ig, ib);
    //     }
    // }
    // eprintln!("Done.");
}
