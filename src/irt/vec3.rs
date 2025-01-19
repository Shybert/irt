use crate::Point;
use rand::prelude::*;
use std::ops::{Add, Div, Mul, Neg, Sub};

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

    pub fn length_squared(&self) -> f32 {
        return self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
    }

    pub fn length(&self) -> f32 {
        return self.length_squared().sqrt();
    }

    pub fn normalize(&self) -> Self {
        return *self / self.length();
    }

    pub fn near_zero(&self) -> bool {
        let threshold = 1e-8;
        return self.x.abs() < threshold && self.y.abs() < threshold && self.z.abs() < threshold;
    }

    pub fn dot(&self, other: &Self) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn cross(&self, other: &Self) -> Self {
        return Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        );
    }

    pub fn reflect(&self, other: &Self) -> Self {
        return *self - 2. * self.dot(other) * *other;
    }

    pub fn random() -> Self {
        let mut rng = thread_rng();
        return Self::new(rng.gen(), rng.gen(), rng.gen());
    }

    pub fn random_interval(min: f32, max: f32) -> Self {
        let mut rng = thread_rng();
        return Self::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        );
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let random_vector = Self::random_interval(-1., 1.);
            let length_squared = random_vector.length_squared();
            if (1e-120..=1.).contains(&length_squared) {
                return random_vector.normalize();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Self) -> Self {
        let vector_on_unit_sphere = Self::random_unit_vector();
        if vector_on_unit_sphere.dot(normal) > 0. {
            return vector_on_unit_sphere;
        }
        return -vector_on_unit_sphere;
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
