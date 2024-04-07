use std::ops::{AddAssign, MulAssign};

type Point3 = Vec3<f64>;
pub struct Vec3<T>(T, T, T);

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(x, y, z)
    }

    pub fn x(&self) -> &T {
        &self.0
    }

    pub fn y(&self) -> &T {
        &self.1
    }

    pub fn z(&self) -> &T {
        &self.2
    }
}

impl<T: MulAssign + Copy> Vec3<T> {
    pub fn scale(&mut self, t: T) {
        self.0 *= t;
        self.1 *= t;
        self.2 *= t;
    }
}

impl Vec3<f64> {
    fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
    pub fn len(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }
}

impl<T: AddAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}
