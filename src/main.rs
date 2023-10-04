use crate::vec3::Point3;
use crate::sphere::Sphere;
use crate::hittable_list::HittableList;
use crate::camera::Camera;


mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod rtweekend;
mod interval;
mod camera;

use crate::vec3::Vec3;

fn main() {

    let mut world = HittableList::new_default();
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5)
    ));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0)
    ));

    let mut camera = Camera::new_default();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.render(&world);
}
