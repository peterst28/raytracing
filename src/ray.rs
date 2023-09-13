use crate::vec3::Vec3;
use crate::vec3::Point3;

// mod vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new_default() -> Ray {
        Ray {
            orig : Vec3::new_default(),
            dir: Vec3::new_default()
        }
    }

    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction
        }
    }

    pub fn origin(&self) -> Vec3 {
        return self.orig
    }

    pub fn direction(&self) -> Vec3 {
        return self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        return self.orig + t * self.dir
    }

}