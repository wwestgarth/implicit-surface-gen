use crate::vec3::{Point3, Vec3, unit_x, unit_y, unit_z};

#[derive(Default)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}


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
}


#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

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

}