use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::vec3::Vec3;
use crate::vec3::Point3;
use crate::ray::Ray;


pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new_default() -> Sphere {
        Sphere {
            center: Point3::new_default(),
            radius: 0.0,
        }
    }

    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> (bool, HitRecord) {
        let mut rec = HitRecord::new_default();
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b*half_b - a*c;

        if discriminant < 0.0 {
            return (false, rec);
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (-half_b + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return (false, rec);
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        return (true, rec);
    }
} 