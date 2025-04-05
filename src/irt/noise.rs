use std::array;

use rand::seq::SliceRandom;
use rand::{random, thread_rng, Rng};

use super::{lerp, Point};

#[derive(Debug)]
pub struct Perlin {
    random_float: [f32; 256],
    permutation: [usize; 512],
}
impl Perlin {
    pub fn new() -> Self {
        let mut rng = thread_rng();

        let mut perm2: [usize; 256] = array::from_fn(|i| i);
        perm2.shuffle(&mut rng);
        let mut perm = [0; 512];
        for i in 0..256 {
            perm[i] = perm2[i];
            perm[i + 256] = perm2[i];
        }

        return Self {
            random_float: array::from_fn(|_| rng.gen()),
            permutation: perm,
        };
    }

    pub fn noise(&self, point: &Point) -> f32 {
        let cube_x = ((point.x.floor() as i32) & 255) as usize;
        let cube_y = ((point.y.floor() as i32) & 255) as usize;
        let cube_z = ((point.z.floor() as i32) & 255) as usize;

        let x = point.x - point.x.floor();
        let y = point.y - point.y.floor();
        let z = point.z - point.z.floor();

        let aaa = self.permutation[self.permutation[self.permutation[cube_x] + cube_y] + cube_z];
        let aba =
            self.permutation[self.permutation[self.permutation[cube_x] + cube_y + 1] + cube_z];
        let aab =
            self.permutation[self.permutation[self.permutation[cube_x] + cube_y] + cube_z + 1];
        let abb =
            self.permutation[self.permutation[self.permutation[cube_x] + cube_y + 1] + cube_z + 1];
        let baa =
            self.permutation[self.permutation[self.permutation[cube_x + 1] + cube_y] + cube_z];
        let bba =
            self.permutation[self.permutation[self.permutation[cube_x + 1] + cube_y + 1] + cube_z];
        let bab =
            self.permutation[self.permutation[self.permutation[cube_x + 1] + cube_y] + cube_z + 1];
        let bbb = self.permutation
            [self.permutation[self.permutation[cube_x + 1] + cube_y + 1] + cube_z + 1];

        return Perlin::trilinear_interpolation(
            [
                self.random_float[aaa],
                self.random_float[baa],
                self.random_float[aba],
                self.random_float[bba],
                self.random_float[aab],
                self.random_float[bab],
                self.random_float[abb],
                self.random_float[bbb],
            ],
            x,
            y,
            z,
        );
    }

    pub fn grad(&self, value: i32, x: f32, y: f32, z: f32) -> f32 {
        let hash = value & 15;
        match hash {
            0 => x + y,
            1 => -x + y,
            2 => x - y,
            3 => -x - y,
            4 => x + z,
            5 => -x + z,
            6 => x - z,
            7 => -x - z,
            8 => y + z,
            9 => -y + z,
            10 => y - z,
            11 => -y - z,
            12 => y + x,
            13 => -y + z,
            14 => y - x,
            15 => -y - z,
            _ => unreachable!(),
        }
    }

    pub fn trilinear_interpolation(cube: [f32; 8], x: f32, y: f32, z: f32) -> f32 {
        let c00 = lerp(cube[0], cube[1], x);
        let c01 = lerp(cube[2], cube[3], x);
        let c10 = lerp(cube[4], cube[5], x);
        let c11 = lerp(cube[6], cube[7], x);

        let c0 = lerp(c00, c01, y);
        let c1 = lerp(c10, c11, y);

        return lerp(c0, c1, z);
    }
}
