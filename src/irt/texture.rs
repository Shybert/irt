use crate::Point;

use crate::Color;
use std::fmt::Debug;

pub trait Texture: Debug + Sync + Send {
    fn value(&self, u: f32, v: f32, point: Point) -> Color;
}

impl Texture for Color {
    fn value(&self, _u: f32, _v: f32, _point: Point) -> Color {
        return *self;
    }
}

#[derive(Debug)]
pub struct CheckeredTexture {
    scale_inverted: f32,
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}
impl CheckeredTexture {
    pub fn new(scale: f32, even: Box<dyn Texture>, odd: Box<dyn Texture>) -> Self {
        return Self {
            scale_inverted: 1. / scale,
            even,
            odd,
        };
    }
}
impl Texture for CheckeredTexture {
    fn value(&self, u: f32, v: f32, point: Point) -> Color {
        let x: i32 = (point.x * self.scale_inverted).floor() as i32;
        let y: i32 = (point.y * self.scale_inverted).floor() as i32;
        let z: i32 = (point.z * self.scale_inverted).floor() as i32;

        let is_even = (x + y + z) % 2 == 0;
        return if is_even {
            self.even.value(u, v, point)
        } else {
            self.odd.value(u, v, point)
        };
    }
}
