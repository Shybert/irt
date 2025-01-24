use std::ops::{Add, AddAssign, Mul, MulAssign};

use rand::prelude::*;

use crate::Interval;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        return Self { r, g, b };
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
impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b);
    }
}
impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}
impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        return Self::new(self.r * scalar, self.g * scalar, self.b * scalar);
    }
}
impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, scalar: f32) {
        *self = Self::new(self.r * scalar, self.g * scalar, self.b * scalar);
    }
}
impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        return color * self;
    }
}
impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, other: Color) -> Self::Output {
        return Self::new(self.r * other.r, self.g * other.g, self.b * other.b);
    }
}

pub fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0. {
        return linear_component.sqrt();
    }
    return 0.;
}
