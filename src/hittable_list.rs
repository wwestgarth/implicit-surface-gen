use std::borrow::Borrow;

use crate::hittable::{HitRecord, Hittable, ImplicitSurface};
use crate::ray::Ray;

#[derive(Default)]
pub struct HittableList {
    parameteric_surfs: Vec<Box<dyn Hittable>>,
    implicit_surfs: Vec<Box<dyn ImplicitSurface>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        Default::default()
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.parameteric_surfs.push(object);
    }

    pub fn add_implicit(&mut self, object: Box<dyn ImplicitSurface>) {
        self.implicit_surfs.push(object);
    }
}

impl HittableList {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        let mut rec = HitRecord::new();

        for object in &self.parameteric_surfs {
            match object.hit(ray, t_min, closest_so_far) {
                None => {}
                Some(trec) => {
                    hit_anything = true;
                    closest_so_far = trec.t;
                    rec = trec.clone();
                }
            }
        }

        for object in &self.implicit_surfs {
            match ray.trace(object.borrow(), t_min, closest_so_far) {
                None => {}
                Some(trec) => {
                    hit_anything = true;
                    closest_so_far = trec.t;
                    rec = trec.clone();
                }
            }
        }

        if !hit_anything {
            return None;
        }
        Some(rec)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::INFINITY;

    use crate::vec3::{eq, Point3};
    use crate::{sphere::Sphere, vec3::origin, vec3::unit_y};
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_multi_hit_implicit() {
        let mut world = HittableList::new();

        // spheres radius 1 at the (0, 5, 0), and (0, 10, 0)
        world.add_implicit(Box::new(Sphere::new(Point3::new(0.0, 5.0, 0.0), 1.0)));
        world.add_implicit(Box::new(Sphere::new(Point3::new(0.0, 10.0, 0.0), 1.0)));

        // if we shoot a ray from from the origin in the y direction we should hit at (0, 4, 0)
        let expect = 4.0 * unit_y();

        let ray = Ray::new(origin(), unit_y());
        let rec = world.hit(&ray, 0.0, INFINITY).unwrap();

        assert!(eq(rec.p, expect));
    }
}
