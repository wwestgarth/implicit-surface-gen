use std::io::Write;

use crate::vec3::Vec3;


pub type Colour = Vec3;

pub fn write_colour(out: &mut impl Write, pixel: Colour) {

    let r = (255.999 * pixel.x()) as i32;
    let g = (255.999 * pixel.y()) as i32;
    let b = (255.999 * pixel.z()) as i32;
    writeln!(out, "{} {} {}", r, g, b).expect("writing color");

}
