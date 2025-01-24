use crate::{Axis, Vec3};
use std::ops::{Add, Index, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        return Point { x, y, z };
    }

    pub fn random() -> Self {
        return Self::new(rand::random(), rand::random(), rand::random());
    }

    /// Returns a point where each element is the smallest of the corresponding elements from each
    /// input.
    pub fn min(&self, other: &Self) -> Self {
        return Self::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        );
    }

    /// Returns a point where each element is the largest of the corresponding elements from each
    /// input.
    pub fn max(&self, other: &Self) -> Self {
        return Self::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        );
    }
}
impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Self::new(self.x + rhs.x, self.y + rhs.y, self.z - rhs.z);
    }
}
impl Add<Vec3> for Point {
    type Output = Self;

    fn add(self, vec3: Vec3) -> Self::Output {
        return Self::new(self.x + vec3.x, self.y + vec3.y, self.z + vec3.z);
    }
}
impl Sub for Point {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        return Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
    }
}
impl Sub<Vec3> for Point {
    type Output = Self;

    fn sub(self, vec3: Vec3) -> Self::Output {
        return Self::new(self.x - vec3.x, self.y - vec3.y, self.z - vec3.z);
    }
}
impl Mul<f32> for Point {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        return Self::new(self.x * scalar, self.y * scalar, self.z * scalar);
    }
}
impl Mul<Point> for f32 {
    type Output = Point;

    fn mul(self, vec3: Point) -> Self::Output {
        return vec3 * self;
    }
}
impl Index<&Axis> for Point {
    type Output = f32;

    fn index(&self, axis: &Axis) -> &Self::Output {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }
}
