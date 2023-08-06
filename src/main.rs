use crate::vec3::Vec3;
use crate::color::Color;
use crate::point3::Point3;

mod vec3;
mod color;
mod point3;

fn main() {

    let image_height: u32 = 256;
    let image_width: u32 = 256;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height-1).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let pixel_color = Vec3::new(
                i as f64 / (image_width-1) as f64,
                j as f64 / (image_height-1) as f64,
                0.25
            );

            println!("{}", pixel_color.write_color());
        }
    }
    eprintln!("Done.");
}
