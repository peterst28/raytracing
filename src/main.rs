fn main() {

    let image_height: u32 = 256;
    let image_width: u32 = 256;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height-1).rev() {
        for i in 0..image_width {
            let r: f64 = i as f64 / (image_width-1) as f64;
            let g: f64 = j as f64 / (image_height-1) as f64;
            let b: f64 = 0.25;

            let ir: u32 = (255.999 * r) as u32;
            let ig: u32 = (255.999 * g) as u32;
            let ib: u32 = (255.999 * b) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
