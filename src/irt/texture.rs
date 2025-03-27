use crate::Point;

use crate::Color;
use std::fmt::Debug;

pub trait Texture: Debug {
    fn value(&self, u: f32, v: f32, point: Point) -> Color;
}

#[derive(Debug)]
pub struct SolidColor {
    albedo: Color,
}
impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        return Self { albedo };
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _point: Point) -> Color {
        return self.albedo;
    }
}
