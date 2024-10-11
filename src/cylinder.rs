use crate::hittable::ImplicitSurface;
use crate::vec3::{self, cross, normalise, Point3, Vec3};

#[derive(Copy, Clone, Default)]
pub struct Cylinder {
    p: Point3,
    dir: Vec3,
    radius: f64,
}

impl Cylinder {
    pub fn new(p: Point3, dir: Vec3, r: f64) -> Cylinder {
        Cylinder {
            p,
            dir: normalise(dir),
            radius: r,
        }
    }
}

impl ImplicitSurface for Cylinder {
    // signed distance function for a cylinder is the closest distance to its spine minus its radius
    //
    // d = (| v - p | X d ) - r
    //
    fn signed_distance(&self, v: Vec3) -> f64 {
        cross(v - self.p, self.dir).length() - self.radius
    }

    // if q is the closet point to the cylinder spine from v, then v - p is the gradient
    fn gradient(&self, v: Vec3) -> Vec3 {
        let l = cross(v - self.p, self.dir).length();
        let h = (v - self.p).length();
        let closest_point = self.p + (self.dir * f64::sqrt((h * h) - (l * l)));

        v - closest_point
    }
}

mod tests {
    use approx::assert_relative_eq;
    use vec3::{origin, unit_x, unit_y, unit_z};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_cylinder_sdf_coi() {
        let s = Cylinder::new(origin(), unit_z(), 1.0);
        assert_relative_eq!(s.signed_distance(unit_x()), 0.0);
        assert_relative_eq!(s.signed_distance(unit_y()), 0.0);
        assert_relative_eq!(s.signed_distance(unit_z()), -1.0);
    }

    #[test]
    fn test_cylinder_external_internal() {
        let s = Cylinder::new(origin(), unit_z(), 5.0);
        assert_relative_eq!(s.signed_distance(10.0 * unit_x()), 5.0);
        assert_relative_eq!(s.signed_distance(unit_x()), -4.0);

        // none aligned with the orgin
        assert_relative_eq!(s.signed_distance(10.0 * unit_x() + unit_z()), 5.0);
        assert_relative_eq!(s.signed_distance(unit_x() + unit_z()), -4.0);
    }

    #[test]
    fn test_cylinder_gradient() {
        let s = Cylinder::new(origin(), unit_z(), 1.0);

        let v = s.gradient(10.0 * unit_x() + unit_z());

        println!("{}", v);
        assert_relative_eq!(v.x(), 10.0);
        assert_relative_eq!(v.y(), 0.0);
        assert_relative_eq!(v.z(), 0.0);
    }
}
