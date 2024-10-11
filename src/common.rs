use rand::Rng;

// scalar values are equal if within this tolerance of each other
const SCALAR_TOL: f64 = 0.00000001;

// Utility functions

pub fn random_double() -> f64 {
    // Return a random real in [0.0, 1.0)
    rand::thread_rng().gen()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    // Return a random real in [min, max)
    min + (max - min) * random_double()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

pub fn scalar_zero(u: f64) -> bool {
    f64::abs(u) < SCALAR_TOL
}
