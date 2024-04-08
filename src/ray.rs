use crate::vec3::{Point3, Vector3};

pub struct Ray<'a, 'b> {
    origin: &'a Point3,
    direction: &'b Vector3,
}

impl<'a, 'b> Ray<'a, 'b> {
    pub fn new(origin: &'a Point3, direction: &'b Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Point3 {
        self.origin
    }

    pub fn direction(&self) -> &Vector3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        let scaled = *self.direction * t;
        *self.origin + scaled
    }
}
