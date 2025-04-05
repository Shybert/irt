use std::array;

use rand::seq::SliceRandom;
use rand::thread_rng;

use super::{lerp, Point};

/// Implementation of improved Perlin noise.
/// Implemented with the help of these resources:
/// <https://raytracing.github.io/books/RayTracingTheNextWeek.html>
/// <https://adrianb.io/2014/08/09/perlinnoise.html>
/// <https://riven8192.blogspot.com/2010/08/calculate-perlinnoise-twice-as-fast.html>
/// <https://mrl.cs.nyu.edu/~perlin/noise/>
#[derive(Debug)]
pub struct Perlin {
    /// The permutation table. It contains all integers from 0..=255,
    /// in a random order, repeated once.
    p: [usize; 512],
}
impl Perlin {
    pub fn new() -> Self {
        let mut rng = thread_rng();

        let mut permutation = [0; 512];
        let (lower, upper) = permutation.split_at_mut(256);
        lower.copy_from_slice(&array::from_fn::<usize, 256, _>(|i| i));
        lower.shuffle(&mut rng);
        upper.copy_from_slice(lower);

        return Self { p: permutation };
    }

    pub fn noise(&self, point: &Point) -> f32 {
        let cube_x = ((point.x.floor() as i32) & 255) as usize;
        let cube_y = ((point.y.floor() as i32) & 255) as usize;
        let cube_z = ((point.z.floor() as i32) & 255) as usize;

        let x = point.x - point.x.floor();
        let y = point.y - point.y.floor();
        let z = point.z - point.z.floor();
        let u = Perlin::fade(x);
        let v = Perlin::fade(y);
        let w = Perlin::fade(z);

        let aaa = self.p[self.p[self.p[cube_x] + cube_y] + cube_z];
        let aba = self.p[self.p[self.p[cube_x] + cube_y + 1] + cube_z];
        let aab = self.p[self.p[self.p[cube_x] + cube_y] + cube_z + 1];
        let abb = self.p[self.p[self.p[cube_x] + cube_y + 1] + cube_z + 1];
        let baa = self.p[self.p[self.p[cube_x + 1] + cube_y] + cube_z];
        let bba = self.p[self.p[self.p[cube_x + 1] + cube_y + 1] + cube_z];
        let bab = self.p[self.p[self.p[cube_x + 1] + cube_y] + cube_z + 1];
        let bbb = self.p[self.p[self.p[cube_x + 1] + cube_y + 1] + cube_z + 1];

        return Perlin::trilinear_interpolation(
            [
                self.gradient(aaa, x, y, z),
                self.gradient(baa, x - 1., y, z),
                self.gradient(aba, x, y - 1., z),
                self.gradient(bba, x - 1., y - 1., z),
                self.gradient(aab, x, y, z - 1.),
                self.gradient(bab, x - 1., y, z - 1.),
                self.gradient(abb, x, y - 1., z - 1.),
                self.gradient(bbb, x - 1., y - 1., z - 1.),
            ],
            u,
            v,
            w,
        );
    }

    fn fade(t: f32) -> f32 {
        return t * t * t * (t * (t * 6. - 15.) + 10.);
    }

    fn gradient(&self, value: usize, x: f32, y: f32, z: f32) -> f32 {
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

    fn trilinear_interpolation(cube: [f32; 8], x: f32, y: f32, z: f32) -> f32 {
        let c00 = lerp(cube[0], cube[1], x);
        let c01 = lerp(cube[2], cube[3], x);
        let c10 = lerp(cube[4], cube[5], x);
        let c11 = lerp(cube[6], cube[7], x);

        let c0 = lerp(c00, c01, y);
        let c1 = lerp(c10, c11, y);

        return lerp(c0, c1, z);
    }
}
