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
        let index = (point.x.abs().floor() as usize) % 256;
        return self.permutation[index];
    }
}
