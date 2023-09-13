use crate::vec3::Vec3;
use crate::vec3::Point3;
use crate::ray::Ray;
use crate::interval::Interval;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {

    pub fn new_default() -> HitRecord {
        return HitRecord {
            p: Point3::new_default(),
            normal: Vec3::new_default(),
            t: 0.0,
            front_face: true
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = Vec3::dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t:&Interval) -> (bool, HitRecord);
}