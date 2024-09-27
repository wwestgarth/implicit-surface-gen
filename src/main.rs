use colour::{write_colour, Colour};
use std::io;

mod vec3;
mod colour;
mod ray;

use ray::Ray;
use vec3::{Point3, Vec3};
 
fn ray_color(r: &Ray) -> Colour {
    let n = vec3::normalise(r.direction());
    let t = 0.5 * (n.y() + 1.0);
    (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
}

fn main() {


    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
 
    // Camera
 
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;
 
    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);


    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);


    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r);
            write_colour(&mut io::stdout(), pixel_color);
        }
    }

    eprint!("\nDone!\n")

}
