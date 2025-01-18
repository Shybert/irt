use crate::{Color, Hittable, Interval, Point, Ray, Vec3};
use std::fs::File;
use std::io::Write;

pub struct Camera {
    aspect_ratio: f32,
    image_width: u32,
    image_height: u32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: u32,
    pixel_samples_scale: f32,
    max_depth: u32,
}
impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u32, samples_per_pixel: u32) -> Self {
        // Image height should be at least 1
        let mut image_height = (image_width as f32 / aspect_ratio) as u32;
        image_height = if image_height < 1 { 1 } else { image_height };

        // Camera
        let focal_length = 1.;
        let viewport_height = 2.;
        let viewport_width = viewport_height * ((image_width as f32) / image_height as f32);
        let camera_center = Point::new(0., 0., 0.);

        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_upper_left =
            camera_center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let pixel_samples_scale = 1. / samples_per_pixel as f32;

        return Self {
            aspect_ratio,
            image_width,
            image_height,
            center: camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth: 10,
        };
    }

    fn ray_color(&self, ray: &Ray, depth: u32, world: &impl Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0., 0., 0.);
        }

        let potential_hit = world.hit(ray, &Interval::new(0.001, f32::INFINITY));
        if let Some(hit) = potential_hit {
            let direction = hit.normal + Vec3::random_unit_vector();
            return 0.5 * self.ray_color(&Ray::new(hit.point, direction), depth - 1, world);
        }

        let a = 0.5 * (ray.direction.normalize().y + 1.);
        return (1. - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.0);
    }

    /// Returns the `x` and `y` coordinates of a random point
    /// in the `[-.5,-.5]-[+.5,+.5]` unit square.
    fn sample_square(&self) -> (f32, f32) {
        return (rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5);
    }

    /// Returns a camera ray from the origin
    /// to a randomly sampled point around pixel location `(i, j)`
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let (offset_x, offset_y) = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f32 + offset_x) * self.pixel_delta_u)
            + ((j as f32 + offset_y) * self.pixel_delta_v);

        return Ray::new(self.center, pixel_sample - self.center);
    }

    pub fn render(&self, world: &impl Hittable) {
        let mut image_file = File::create("image.ppm").expect("Could not create image file.");
        writeln!(
            image_file,
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        )
        .unwrap();

        for j in 0..self.image_height {
            println!("Lines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut color = Color::new(0., 0., 0.);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    color += self.ray_color(&ray, self.max_depth, world);
                }
                color *= self.pixel_samples_scale;

                let intensity = Interval::new(0., 0.999);
                let ir = (intensity.clamp(color.r) * 256.) as u32;
                let ig = (intensity.clamp(color.g) * 256.) as u32;
                let ib = (intensity.clamp(color.b) * 256.) as u32;

                writeln!(image_file, "{} {} {}\n", ir, ig, ib).unwrap();
            }
        }

        println!("Print finished.");
    }
}
