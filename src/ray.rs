use crate::{hittable::{ImplicitSurface, HitRecord}, vec3::{normalise, origin, Point3, Vec3}};

#[derive(Default)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}


const SCALAR_TOL: f64 =  0.0001;

impl Ray {
    pub fn new(o: Point3, d: Vec3) -> Self {
        Ray{
            origin:o, 
            direction: d
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


    pub fn trace(&self, su: impl ImplicitSurface, t_min: f64, t_max: f64, rec: &mut HitRecord) {


        // let step along the ray until our distance function changes sign
        let step = (t_max - t_min) / 10.0;


        let initial = su.signed_distance(self.at(0.0));

        println!("distance initially {}", initial);

        let mut i = 1;
        loop {

            let t = i as f64 * step;

            let v = self.at(t);

            println!("stepped to {}", v);

            let d = su.signed_distance(v);
            println!("distance is {}", d);




            // basic: assume out steps are small enough that we kind of hit a 0 when we sign changes,
            // we can refine this later
            if (d < SCALAR_TOL) || d.signum() != initial.signum() {

                rec.t = t;
                rec.p = v;

                // TODO normal of signed distance function.....
                rec.normal = normalise(su.gradient(v));
        
                // convention we have chosen
                rec.set_face_normal(self, rec.normal);
                return

            }


            if i > 10 {
                break
            }
            i += 1;
        }







    }



}


#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use crate::{sphere::Sphere, vec3::unit_x, common::INFINITY};
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_ray_basic() {

        let r = Ray::new( unit_x(), unit_x());

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
        assert_relative_eq!(2.5, at.x());
        assert_relative_eq!(2.0, at.y());
        assert_relative_eq!(2.5, at.z());

        let at = r.at(10.0);
        assert_relative_eq!(31.0, at.x());
        assert_relative_eq!(40.0, at.y());
        assert_relative_eq!(50.0, at.z());
    }

    #[test]
    fn test_ray_sdf() {


        // sphere radius 1 at (5, 0, 0)
        let sphere = Sphere::new(5.0 * unit_x(), 1.0);

        // ray origin at origin pointed along x axis
        let r = Ray::new(origin(), unit_x());

        let mut rec = HitRecord::new();
        r.trace(sphere, 0 as f64, 10 as f64, &mut rec);

        assert_relative_eq!(rec.p.x(), 4.0);
        assert_relative_eq!(rec.p.y(), 0.0);
        assert_relative_eq!(rec.p.z(), 0.0);

        assert_relative_eq!(rec.normal.x(), -1.0);
        assert_relative_eq!(rec.normal.y(), 0.0);
        assert_relative_eq!(rec.normal.z(), 0.0);

    }


}