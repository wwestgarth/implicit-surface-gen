use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    e: [f64;3],
}


pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3{ e: [x,y,z]}
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        (self.x() * self.x()) + (self.y() * self.y()) + (self.z() * self.z())
    }

}




// Output formatting
impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

// Vec3 + Vec3
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 { 
        Vec3::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
    }

}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v
    }
}


impl Sub for Vec3 {
    type Output = Vec3;
 
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, v: Vec3) {
        *self = *self - v
    }
}


// f64 * Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;
 
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x(), self * v.y(), self * v.z())
    }
}
 
// Vec3 * f64
impl Mul<f64> for Vec3 {
    type Output = Vec3;
 
    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.x() * t, self.y() * t, self.z() * t)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
 
    fn div(self, t: f64) -> Vec3 {
        Vec3::new(self.x() / t, self.y() / t, self.z() / t)
    }
}

pub fn dot(u:Vec3, v: Vec3) -> f64{
        u.x() * v.x() +
        u.y() * v.y() +
        u.z() * v.z()
}

pub fn cross(u:Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.y() * v.z() - u.z() * v.y(),
        u.z() * v.x() - u.x() * v.z(),
        u.x() * v.y() - u.y() * v.x(),
    )
}

pub fn normalise(u:Vec3) -> Vec3 {
    u / u.length()
}



pub fn unit_x() -> Vec3 {
    Vec3::new(1.0, 0.0, 0.0)
}

pub fn unit_y() -> Vec3 {
    Vec3::new(0.0, 1.0, 0.0)
}

pub fn unit_z() -> Vec3 {
    Vec3::new(0.0, 0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_vec_unit_length() {

        let v = unit_x();
        assert_relative_eq!(v.length_squared(), 1.0);
        assert_relative_eq!(v.length(), 1.0);

        let v = unit_y();
        assert_relative_eq!(v.length_squared(), 1.0);
        assert_relative_eq!(v.length(), 1.0);

        let v = unit_z();
        assert_relative_eq!(v.length_squared(), 1.0);
        assert_relative_eq!(v.length(), 1.0);
    }

    #[test]
    fn test_vec_rt2_length() {

        let v = unit_x() + unit_y();
        assert_relative_eq!(v.length_squared(), 2.0);
        assert_relative_eq!(v.length(), 1.4142135623730951);

        let v = unit_y() + unit_z();
        assert_relative_eq!(v.length_squared(), 2.0);
        assert_relative_eq!(v.length(), 1.4142135623730951);

        let v = unit_x() + unit_z();
        assert_relative_eq!(v.length_squared(), 2.0);
        assert_relative_eq!(v.length(), 1.4142135623730951);
    }

    #[test]
    fn test_vec_neg() {

        let v = Vec3::new(1.0, 2.0, 3.0);
        let u = -v;

        assert_relative_eq!(-1.0, u.x());
        assert_relative_eq!(-2.0, u.y());
        assert_relative_eq!(-3.0, u.z());
    }

    #[test]
    fn test_vec_add() {

        let v = Vec3::new(1.0, 2.0, 3.0);
        let u = Vec3::new(4.0, 5.0, 6.0);

        let mut w = v + u;
        assert_relative_eq!(5.0, w.x());
        assert_relative_eq!(7.0, w.y());
        assert_relative_eq!(9.0, w.z());

        // w plus its negation should be 0
        w += -w;
        assert_relative_eq!(0.0, w.x());
        assert_relative_eq!(0.0, w.y());
        assert_relative_eq!(0.0, w.z());
    }

    #[test]
    fn test_vec_dot() {
        let v = Vec3::new(3.0, 4.0, 5.0);
        assert_relative_eq!(v.length_squared(), dot(v,v));
    }


    #[test]
    fn test_vec_cross() {
        let u = unit_x();
        let v = unit_y();

        let w = cross(u, v);
        assert_relative_eq!(0.0, w.x());
        assert_relative_eq!(0.0, w.y());
        assert_relative_eq!(1.0, w.z());

        let w = cross(v, u);
        assert_relative_eq!(0.0, w.x());
        assert_relative_eq!(0.0, w.y());
        assert_relative_eq!(-1.0, w.z());
    }

    #[test]
    fn test_vec_normalise() {
        let u = 5.0 * unit_x();
        let v = normalise(u);
        assert_relative_eq!(1.0, v.x());
        assert_relative_eq!(0.0, v.y());
        assert_relative_eq!(0.0, v.z());
    }
}