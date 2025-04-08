use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{degrees_to_radians, linear_to_gamma, Color, Hittable, Interval, Point, Ray, Vec3};
use std::fs::File;
use std::io::Write;

struct CameraBasis {
    u: Vec3,
    v: Vec3,
    w: Vec3,
}
impl CameraBasis {
    fn new(look_from: Point, look_at: Point, up: Vec3) -> Self {
        let w = (look_from - look_at).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u).normalize();

        return Self { u, v, w };
    }
}

pub struct Camera {
    #[allow(dead_code)]
    aspect_ratio: f32,
    image_width: u32,
    image_height: u32,
    #[allow(dead_code)]
    vertical_fov: f32,
    center: Point,
    #[allow(dead_code)]
    basis: CameraBasis,
    pixel00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: u32,
    pixel_samples_scale: f32,
    max_depth: u32,
    background: Color,
}
impl Camera {
    pub fn new(
        aspect_ratio: f32,
        vertical_fov: f32,
        image_width: u32,
        look_from: Point,
        look_at: Point,
        up: Vec3,
        samples_per_pixel: u32,
        background: Color,
    ) -> Self {
        // Image height should be at least 1
        let mut image_height = (image_width as f32 / aspect_ratio) as u32;
        image_height = if image_height < 1 { 1 } else { image_height };

        let focal_length = (look_from - look_at).length();
        let basis = CameraBasis::new(look_from, look_at, up);
        let center = look_from;

        let theta = degrees_to_radians(vertical_fov);
        let h = f32::tan(theta / 2.);
        let viewport_height = 2. * h * focal_length;
        let viewport_width = viewport_height * ((image_width as f32) / image_height as f32);

        let viewport_u = viewport_width * basis.u;
        let viewport_v = viewport_height * -basis.v;

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_upper_left =
            center - (focal_length * basis.w) - viewport_u / 2. - viewport_v / 2.;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let pixel_samples_scale = 1. / samples_per_pixel as f32;

        return Self {
            aspect_ratio,
            image_width,
            image_height,
            vertical_fov,
            center,
            basis,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth: 10,
            background,
        };
    }

    fn ray_color(&self, ray: &Ray, depth: u32, world: &impl Hittable) -> Color {
        if depth == 0 {
            return Color::black();
        }

        let potential_hit = world.hit(ray, &mut Interval::new(0.001, f32::INFINITY));
        let Some(hit) = potential_hit else {
            return self.background;
        };

        let color_from_emission = hit.material.emitted(hit.u, hit.v, hit.point);

        let potential_scatter = hit.material.scatter(ray, &hit);
        let Some((scattered, attenuation)) = potential_scatter else {
            return color_from_emission;
        };
        let color_from_scatter = attenuation * self.ray_color(&scattered, depth - 1, world);

        return color_from_emission + color_from_scatter;
    }

    /// Returns the `x` and `y` coordinates of a random point
    /// in the `[-.5,-.5]-[+.5,+.5]` unit square.
    fn sample_square(&self) -> (f32, f32) {
        return (rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5);
    }

    /// Returns a camera ray from the origin
    /// to a randomly sampled point around pixel location `(x, y)`
    fn get_ray(&self, (x, y): (u32, u32)) -> Ray {
        let (offset_x, offset_y) = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((x as f32 + offset_x) * self.pixel_delta_u)
            + ((y as f32 + offset_y) * self.pixel_delta_v);

        return Ray::new(self.center, pixel_sample - self.center);
    }

    fn coords_from_index(&self, index: u32) -> (u32, u32) {
        let x = index % self.image_width;
        let y = index / self.image_width;
        return (x, y);
    }

    /// Samples the pixel given by (x, y)
    /// `self.samples_per_pixel` times.
    fn sample_pixel(&self, world: &impl Hittable, (x, y): (u32, u32)) -> Color {
        let mut color = Color::black();
        for _ in 0..self.samples_per_pixel {
            let ray = self.get_ray((x, y));
            color += self.ray_color(&ray, self.max_depth, world);
        }

        return color * self.pixel_samples_scale;
    }

    pub fn render(&self, world: &impl Hittable) {
        let mut image_file = File::create("image.ppm").expect("Could not create image file.");
        writeln!(
            image_file,
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        )
        .unwrap();

        let canvas: Vec<Color> = (0..self.image_height * self.image_width)
            .into_par_iter()
            .progress_count((self.image_width * self.image_height).into())
            .map(|index| self.coords_from_index(index))
            .map(|(x, y)| self.sample_pixel(world, (x, y)))
            .collect();

        canvas.iter().for_each(|color| {
            let r = linear_to_gamma(color.r);
            let g = linear_to_gamma(color.g);
            let b = linear_to_gamma(color.b);

            let intensity = Interval::new(0., 0.999);
            let ir = (intensity.clamp(r) * 256.) as u32;
            let ig = (intensity.clamp(g) * 256.) as u32;
            let ib = (intensity.clamp(b) * 256.) as u32;

            writeln!(image_file, "{} {} {}\n", ir, ig, ib).unwrap();
        });

        println!("Print finished.");
    }
}
