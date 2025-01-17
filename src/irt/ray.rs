use crate::{Point, Vec3};

pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        return Self { origin, direction };
    }

    pub fn at(&self, t: f32) -> Point {
        return self.origin + t * self.direction;
    }
}
