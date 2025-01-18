use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        return Self { r, g, b };
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

pub fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0. {
        return linear_component.sqrt();
    }
    return 0.;
}
