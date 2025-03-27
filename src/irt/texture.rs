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
