mod camera;
mod colour;
mod common;
mod cylinder;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;
use anyhow::Result;
use clap::Parser;

mod settings;

use settings::Settings;

use std::fs;
use std::io::Write;

use camera::Camera;
use colour::Colour;
use cylinder::Cylinder;
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::{unit_x, unit_y, unit_z, Point3};

fn ray_color(r: &Ray, world: &HittableList, depth: u64) -> Colour {
    if depth == 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    match world.hit(r, 0.0, f64::INFINITY) {
        None => {}
        Some(rec) => {
            let direction = rec.normal + (0.5 * vec3::random_unit_vector());
            return 0.5 * ray_color(&Ray::new(rec.p, direction), world, depth - 1);
        }
    }

    let n = vec3::normalise(r.direction());
    let t = 0.5 * (n.y() + 1.0);
    (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
}

fn main() -> Result<()> {
    let args = settings::Args::parse();

    let cfg = Settings::new(&args.config)?;

    // let open a file
    let mut file = fs::File::create(cfg.output)?;

    // World

    let mut world = HittableList::new();

    // implictly defined sphere in the middle
    world.add_implicit(Box::new(Sphere::new(Point3::new(0.0, 0.0, -2.0), 0.75)));
    world.add_implicit(Box::new(Cylinder::new(
        Point3::new(0.0, 0.0, -1.0),
        unit_y(),
        0.25,
    )));

    // explicitly defined sphere as the floor
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let cam = Camera::new();

    _ = file.write(format!("P3\n{} {}\n255\n", cfg.view.width, cfg.view.height).as_bytes())?;

    let samples_per_pixel = cfg.sampling.samples_per_pixel;
    for j in (0..cfg.view.height).rev() {
        eprint!("\rScanlines remaining {}", j);
        for i in 0..cfg.view.width {
            let mut pixel_color = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + common::random_double()) / (cfg.view.width - 1) as f64;
                let v = (j as f64 + common::random_double()) / (cfg.view.height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, cfg.sampling.max_depth);
            }
            colour::write_color(&mut file, pixel_color, samples_per_pixel);
        }
    }

    Ok(())
}
