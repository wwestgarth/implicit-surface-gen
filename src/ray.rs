use crate::{
    hittable::{HitRecord, ImplicitSurface},
    vec3::{normalise, Point3, Vec3},
};

#[derive(Default)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

const SCALAR_TOL: f64 = 0.00000001;

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

    pub fn trace(&self, su: &Box<dyn ImplicitSurface>, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // let step along the ray until our distance function changes sign


        let initial = su.signed_distance(self.at(0.0));


        if initial > t_max {
            return false;
        }

        let mut step = initial;

        let mut t = 0.0;

        //println!("distance initially {}", initial);

        let mut i = 1;
        loop {
            t += step;

            let v = self.at(t);

            //println!("stepped to {}", v);

            let d = su.signed_distance(v);
            //println!("distance is {}", d);


            // if we've stepped and the distance has increased, just stop now because we're heading 
            // in the wrong direction
            if d > step {
                return false
            }

            step = d;

            // basic: assume out steps are small enough that we kind of hit a 0 when we sign changes,
            // we can refine this later
            if d < SCALAR_TOL {
                rec.t = t;
                rec.p = v;

                // TODO normal of signed distance function.....
                rec.normal = normalise(su.gradient(v));

                // convention we have chosen
                rec.set_face_normal(self, rec.normal);
                //println!("HIT");
                return true;
            }

            if i > 100 {
                //println!("MISS");
                return false
            }
            i += 1;
        }
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


        let boxed: Box<dyn ImplicitSurface> = Box::new(sphere);

        let mut rec = HitRecord::new();
        r.trace(&boxed, 0 as f64, 10 as f64, &mut rec);

        assert_relative_eq!(rec.p.x(), 4.0);
        assert_relative_eq!(rec.p.y(), 0.0);
        assert_relative_eq!(rec.p.z(), 0.0);

        assert_relative_eq!(rec.normal.x(), -1.0);
        assert_relative_eq!(rec.normal.y(), 0.0);
        assert_relative_eq!(rec.normal.z(), 0.0);
    }
}
