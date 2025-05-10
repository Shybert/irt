use crate::irt::{approx_equals, Axis, Interval, Point};
use rand::prelude::*;
use std::ops::{Add, Div, Index, Mul, Neg, Sub};

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

    pub fn length_squared(self) -> f32 {
        return self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
    }

    pub fn length(self) -> f32 {
        return self.length_squared().sqrt();
    }

    /// Normalizes the vector, returning a [`UnitVec3`] guaranteed to be of unit length.
    pub fn normalize(self) -> UnitVec3 {
        return UnitVec3::new(self);
    }

    pub fn near_zero(self) -> bool {
        let threshold = 1e-8;
        return self.x.abs() < threshold && self.y.abs() < threshold && self.z.abs() < threshold;
    }

    /// Returns a vector where each element is the smallest of the corresponding elements from each
    /// input.
    pub fn min(self, other: Self) -> Self {
        return Self::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        );
    }

    /// Returns the smallest of the vector's components.
    pub fn min_component(self) -> f32 {
        return self.x.min(self.y.min(self.z));
    }

    /// Returns the largest of the vector's components.
    pub fn max_component(self) -> f32 {
        return self.x.max(self.y.max(self.z));
    }

    /// Returns a vector where each element is the largest of the corresponding elements from each
    /// input.
    pub fn max(self, other: Self) -> Self {
        return Self::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        );
    }

    pub fn longest_axis(self) -> Axis {
        if self.x > self.y {
            if self.x > self.z {
                return Axis::X;
            } else {
                return Axis::Z;
            };
        } else if self.y > self.z {
            return Axis::Y;
        } else {
            return Axis::Z;
        };
    }

    pub fn dot(self, other: Self) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn cross(self, other: Self) -> Self {
        return Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        );
    }

    pub fn reflect(self, other: Self) -> Self {
        return self - 2. * self.dot(other) * other;
    }

    pub fn refract(self, normal: Self, refractive_index_ratio: f32) -> Self {
        let cos_theta = self.dot(normal);
        let a = refractive_index_ratio * self;
        let b = refractive_index_ratio * cos_theta;
        let c = (1. - refractive_index_ratio.powi(2) * (1. - (cos_theta).powi(2))).sqrt();

        return a - (b + c) * normal;
    }

    pub fn random() -> Self {
        let mut rng = thread_rng();
        return Self::new(rng.gen(), rng.gen(), rng.gen());
    }

    pub fn random_in_interval(interval: &Interval) -> Self {
        let mut rng = thread_rng();
        return Self::new(
            rng.gen_range(interval.min..interval.max),
            rng.gen_range(interval.min..interval.max),
            rng.gen_range(interval.min..interval.max),
        );
    }
}
impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        return approx_equals(self.x, other.x)
            && approx_equals(self.y, other.y)
            && approx_equals(self.z, other.z);
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
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        return Self::new(-self.x, -self.y, -self.z);
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
impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self::Output {
        return Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z);
    }
}
impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self::Output {
        return self * (1. / scalar);
    }
}
impl Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, vec3: Vec3) -> Self::Output {
        return Vec3::new(self / vec3.x, self / vec3.y, self / vec3.z);
    }
}
impl From<Point> for Vec3 {
    fn from(point: Point) -> Self {
        Self::new(point.x, point.y, point.z)
    }
}
impl Index<&Axis> for Vec3 {
    type Output = f32;

    fn index(&self, axis: &Axis) -> &Self::Output {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }
}

/// [`Vec3`] guaranteed to be of unit length.
#[derive(Debug, Clone, Copy)]
pub struct UnitVec3(Vec3);
impl UnitVec3 {
    pub fn new(vec3: Vec3) -> Self {
        return Self(vec3 / vec3.length());
    }

    /// Creates a [`UnitVec3`] without normalizing the input vector.
    /// Useful when the caller is certain that the input vector is of unit length.
    /// Panics in debug builds if the input is not of unit length.
    fn new_unchecked(vec3: Vec3) -> Self {
        debug_assert!(approx_equals(vec3.length(), 1.));
        return Self(vec3);
    }

    pub fn as_vec3(self) -> Vec3 {
        return self.0;
    }

    pub fn random() -> Self {
        loop {
            let random_vector = Vec3::random_in_interval(&Interval::new(-1., 1.));
            let length_squared = random_vector.length_squared();
            if (1e-120..=1.).contains(&length_squared) {
                return random_vector.normalize();
            }
        }
    }
}
impl Neg for UnitVec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        return Self::new_unchecked(-self.0);
    }
}
