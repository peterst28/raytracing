use crate::rtweekend;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Color;
use crate::vec3::Vec3;
use crate::vec3::Point3;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3
}

impl Camera {
    pub fn new_default() -> Camera {
        return Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            image_height: 0,
            center: Point3::new_default(),
            pixel00_loc: Point3::new_default(),
            pixel_delta_u: Vec3::new_default(),
            pixel_delta_v: Vec3::new_default(),
            max_depth: 10
        };
    }

    pub fn new() -> Camera {
        return Camera::new_default()
    }

    pub fn render(&mut self, world: &HittableList) {
        self.initialize();

        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprintln!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new_default();
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i,j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }

                println!("{}", pixel_color.write_color(self.samples_per_pixel));
            }
        }
        eprintln!("Done.");
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center = self.pixel00_loc 
                                    + (i as f64 * self.pixel_delta_u) 
                                    + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center - self.pixel_sample_square();
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        return Ray::new(self.center, ray_direction);
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + rtweekend::random_double();
        let py = -0.5 + rtweekend::random_double();
        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
    }

    fn initialize(&mut self) {

        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64/ self.image_height as f64) as f64;
        self.center = Point3::new(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.center 
                                            - Vec3::new(0.0, 0.0, focal_length) 
                                            - viewport_u / 2.0 
                                            - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(&self, ray:&Ray, depth: i32, world: &HittableList) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let (has_hit, rec) = world.hit(ray, &Interval::new(0.001, rtweekend::INFINITY));
        if has_hit {
            let direction = rec.normal + Vec3::random_unit_vector();
            return 0.5 * self.ray_color(&Ray::new(rec.p, direction), depth-1, world);
        }

        let unit_direction = Vec3::unit_vector(&ray.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0-a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    }
}