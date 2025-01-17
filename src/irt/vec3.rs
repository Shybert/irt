use crate::Point;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        return Vec3 { x, y, z };
    }

    pub fn magnitude(&self) -> f32 {
        return (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
    }

    pub fn normalize(self) -> Self {
        return self / self.magnitude();
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn random() -> Self {
        return Self::new(rand::random(), rand::random(), rand::random());
    }
}
impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Self::new(self.x + rhs.x, self.y + rhs.y, self.z - rhs.z);
    }
}
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
    }
}
impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        return Self::new(self.x * scalar, self.y * scalar, self.z * scalar);
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, vec3: Vec3) -> Self::Output {
        return vec3 * self;
    }
}
impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self::Output {
        return self * (1. / scalar);
    }
}
impl From<Point> for Vec3 {
    fn from(point: Point) -> Self {
        Self::new(point.x, point.y, point.z)
    }
}
