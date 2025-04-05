use image::Rgb32FImage;
use rand::random;

use crate::Point;

use crate::Color;
use std::fmt::Debug;

use super::Perlin;

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

#[derive(Debug)]
pub struct ImageTexture {
    img: Rgb32FImage,
}
impl ImageTexture {
    pub fn new(image_filename: &str) -> Self {
        return Self {
            img: image::open(image_filename).unwrap().to_rgb32f(),
        };
    }
}
impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _point: Point) -> Color {
        if self.img.width() == 0 || self.img.height() == 0 {
            return Color::cyan();
        }

        let x = (u * self.img.width() as f32) as u32;
        let y = ((1. - v) * self.img.height() as f32) as u32;
        let pixel = self.img.get_pixel(x, y).0;

        return Color::new(pixel[0], pixel[1], pixel[2]);
    }
}

#[derive(Debug)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}
impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        return Self {
            noise: Perlin::new(),
            scale,
        };
    }
}
impl Texture for NoiseTexture {
    fn value(&self, u: f32, v: f32, point: Point) -> Color {
        return Color::white() * 0.5 * (1. + self.noise.noise(&(self.scale * point)));
    }
}
