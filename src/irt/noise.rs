use std::array;

use rand::{random, thread_rng, Rng};

use super::Point;

#[derive(Debug)]
pub struct Perlin {
    permutation: [f32; 256],
}
impl Perlin {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        return Self {
            permutation: array::from_fn(|_| rng.gen()),
        };
    }

    pub fn noise(&self, point: &Point) -> f32 {
        let x = (point.x.floor() as i32) & 255;
        let y = (point.y.floor() as i32) & 255;
        let z = (point.z.floor() as i32) & 255;

        return self.permutation[(x ^ y ^ z) as usize];
    }
}
