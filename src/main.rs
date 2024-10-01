mod colour;
mod common;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;
mod camera;
 
use std::io;
 
use colour::Colour;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::{Point3, Vec3};
use camera::Camera;
 
fn ray_color(r: &Ray, world: &dyn Hittable) -> Colour {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, common::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Colour::new(1.0, 1.0, 1.0));
    }
 
    let unit_direction = vec3::normalise(r.direction());


    let n = vec3::normalise(r.direction());
    let t = 0.5 * (n.y() + 1.0);
    (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
}




fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = vec3::dot(oc, r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - f64::sqrt(discriminant)) / a
    }
}



fn main() {


    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;

    // World
 
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
 
    let cam = Camera::new();

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);


    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + common::random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + common::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            colour::write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprint!("\nDone!\n")

}
