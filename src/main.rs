use crate::vec3::Vec3;

mod vec3;

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
