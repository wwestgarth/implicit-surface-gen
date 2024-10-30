use crate::{
    common::scalar_zero,
    hittable::{HitRecord, ImplicitSurface},
    vec3::{normalise, Point3, Vec3},
};

#[derive(Default)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(o: Point3, d: Vec3) -> Self {
        Ray {
            origin: o,
            direction: normalise(d),
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + (t * self.direction)
    }

    pub fn trace(&self, su: &dyn ImplicitSurface, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut t = t_min;
        let mut dist = su.signed_distance(self.at(t));

        // if the ray is being fired from the surface we assume no hit since it will be an ray from a reflection
        if scalar_zero(dist) {
            return None;
        }

        let mut iteration = 200;
        while iteration > 0 {
            t += dist;

            let v = self.at(t);
            let d = su.signed_distance(v);

            // we've stepped outside of the maximum parameter for the ray
            if t > t_max {
                return None;
            }

            // if we've stepped and the distance has increased, something has gone wrong so we'll just bail for now
            if d > dist {
                return None;
            }

            dist = d;

            if scalar_zero(d) {
                let mut rec = HitRecord::new();
                rec.t = t;
                rec.p = v;

                rec.normal = normalise(su.gradient(v));
                rec.set_face_normal(self, rec.normal);
                return Some(rec);
            }

            iteration -= 1;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{sphere::Sphere, vec3::origin, vec3::unit_x};
    use approx::assert_relative_eq;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_ray_basic() {
        let r = Ray::new(unit_x(), unit_x());

        let at = r.at(0.0);
        assert_relative_eq!(1.0, at.x());
        assert_relative_eq!(0.0, at.y());
        assert_relative_eq!(0.0, at.z());

        let at = r.at(0.5);
        assert_relative_eq!(1.5, at.x());
        assert_relative_eq!(0.0, at.y());
        assert_relative_eq!(0.0, at.z());

        let at = r.at(10.0);
        assert_relative_eq!(11.0, at.x());
        assert_relative_eq!(0.0, at.y());
        assert_relative_eq!(0.0, at.z());
    }

    #[test]
    fn test_ray_funky() {
        let r = Ray::new(unit_x(), Vec3::new(3.0, 4.0, 5.0));

        let at = r.at(0.0);
        assert_relative_eq!(1.0, at.x());
        assert_relative_eq!(0.0, at.y());
        assert_relative_eq!(0.0, at.z());

        let at = r.at(0.5);
        assert_relative_eq!(1.2121320343559643, at.x());
        assert_relative_eq!(0.282842712474619, at.y());
        assert_relative_eq!(0.35355339059327373, at.z());

        let at = r.at(10.0);
        assert_relative_eq!(5.242640687119285, at.x());
        assert_relative_eq!(5.65685424949238, at.y());
        assert_relative_eq!(7.071067811865475, at.z());
    }

    #[test]
    fn test_ray_sdf() {
        // sphere radius 1 at (5, 0, 0)
        let sphere = Sphere::new(5.0 * unit_x(), 1.0);

        // ray origin at origin pointed along x axis
        let r = Ray::new(origin(), unit_x());

        //let boxed: Box<dyn ImplicitSurface> = Box::new(sphere);

        let rec = r.trace(&sphere, 0 as f64, 10 as f64).unwrap();

        assert_relative_eq!(rec.p.x(), 4.0);
        assert_relative_eq!(rec.p.y(), 0.0);
        assert_relative_eq!(rec.p.z(), 0.0);

        assert_relative_eq!(rec.normal.x(), -1.0);
        assert_relative_eq!(rec.normal.y(), 0.0);
        assert_relative_eq!(rec.normal.z(), 0.0);
    }
}
