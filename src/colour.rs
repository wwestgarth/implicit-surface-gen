use crate::common;
use crate::vec3::Vec3;
use std::io::Write;

pub type Colour = Vec3;

pub fn write_color(out: &mut impl Write, pixel_color: Colour, samples_per_pixel: u64) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples and gamma-correct for gamma=2.0
    let scale = 1.0 / samples_per_pixel as f64;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    // Write the translated [0, 255] value of each color component
    writeln!(
        out,
        "{} {} {}",
        (256.0 * common::clamp(r, 0.0, 0.999)) as i32,
        (256.0 * common::clamp(g, 0.0, 0.999)) as i32,
        (256.0 * common::clamp(b, 0.0, 0.999)) as i32,
    )
    .expect("writing color");
}
