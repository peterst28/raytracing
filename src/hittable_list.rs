use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use std::vec::Vec;
use crate::ray::Ray;
use crate::interval::Interval;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {

    pub fn new_default() -> HittableList {
        HittableList {
            objects: Vec::new()
        }
    }

    pub fn new(object: Box<dyn Hittable>) -> HittableList {
        let mut new_hittable_list = HittableList::new_default();
        new_hittable_list.add(object);
        return new_hittable_list;
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> (bool, HitRecord) {
        let mut rec = HitRecord::new_default();
        let mut temp_rec: HitRecord;
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        let mut temp_hit_anything: bool;

        for object in &self.objects {
            (temp_hit_anything, temp_rec) = object.hit(r, &Interval::new(ray_t.min, closest_so_far));
            if temp_hit_anything {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = temp_rec;
            }
        }

        return (hit_anything, rec);
    }
}