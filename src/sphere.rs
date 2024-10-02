use crate::hittable::{HitRecord, Hittable, ImplicitSurface};
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};

#[derive(Copy, Clone, Default)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
        }
    }
}

impl ImplicitSurface for Sphere {
    // signed distance function for a sphere is the distance from the centre minus its radius.
    // if v is outside of the sphere it will have a positive value, negative if inside, and
    // 0 if coincident.
    fn signed_distance(&self, v: Vec3) -> f64 {
        (v - self.center).length() - self.radius
    }

    fn gradient(&self, v: Vec3) -> Vec3 {
        v - self.center
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrt_d) / a;
        if root <= t_min || t_max <= root {
            root = (-half_b + sqrt_d) / a;
            if root <= t_min || t_max <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;

        // convention we have chosen
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        true
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use vec3::{origin, unit_x, unit_y, unit_z};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_sphere_sdf_coi() {
        let s = Sphere::new(origin(), 1.0);
        assert_relative_eq!(s.signed_distance(unit_x()), 0.0);
        assert_relative_eq!(s.signed_distance(unit_y()), 0.0);
        assert_relative_eq!(s.signed_distance(unit_z()), 0.0);
    }

    #[test]
    fn test_sphere_sdf_outside() {
        let s = Sphere::new(origin(), 1.0);
        assert_relative_eq!(s.signed_distance(2.0 * unit_x()), 1.0);
        assert_relative_eq!(s.signed_distance(2.0 * unit_y()), 1.0);
        assert_relative_eq!(s.signed_distance(2.0 * unit_z()), 1.0);
    }

    #[test]
    fn test_sphere_sdf_inside() {
        let s = Sphere::new(origin(), 1.0);
        assert_relative_eq!(s.signed_distance(0.5 * unit_x()), -0.5);
        assert_relative_eq!(s.signed_distance(0.5 * unit_y()), -0.5);
        assert_relative_eq!(s.signed_distance(0.5 * unit_z()), -0.5);

        assert_relative_eq!(s.signed_distance(origin()), -1.0);
    }

    #[test]
    fn test_sphere_sdf_offset() {
        let s = Sphere::new(unit_x(), 1.0);
        assert_relative_eq!(s.signed_distance(unit_x()), -1.0);
        assert_relative_eq!(s.signed_distance(unit_y()), 0.41421356237309515);
        assert_relative_eq!(s.signed_distance(unit_z()), 0.41421356237309515);
    }

    #[test]
    fn test_sphere_gradient() {
        let s = Sphere::new(origin(), 1.0);

        let grad = s.gradient(unit_x());

        assert_relative_eq!(grad.x(), 1.0);
        assert_relative_eq!(grad.y(), 0.0);
        assert_relative_eq!(grad.z(), 0.0);

        let grad = s.gradient(unit_x() + unit_y());

        assert_relative_eq!(grad.x(), 1.0);
        assert_relative_eq!(grad.y(), 1.0);
        assert_relative_eq!(grad.z(), 0.0);
    }
}
