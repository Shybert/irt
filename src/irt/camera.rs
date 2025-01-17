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
}
impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u32) -> Self {
        // let aspect_ratio = 16. / 9.;
        // let image_width = 400;

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

        return Self {
            aspect_ratio,
            image_width,
            image_height,
            center: camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        };
    }

    fn ray_color(&self, ray: &Ray, world: &impl Hittable) -> Color {
        let potential_hit = world.hit(ray, &Interval::new(0., f32::INFINITY));
        if let Some(hit) = potential_hit {
            return 0.5 * Color::new(hit.normal.x + 1., hit.normal.y + 1., hit.normal.z + 1.);
        }

        let a = 0.5 * (ray.direction.normalize().y + 1.);
        return (1. - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.0);
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
                let pixel_center = self.pixel00_loc
                    + (i as f32 * self.pixel_delta_u)
                    + (j as f32 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                let color = self.ray_color(&ray, world);

                let ir = (255.999 * color.r) as u32;
                let ig = (255.999 * color.g) as u32;
                let ib = (255.999 * color.b) as u32;

                writeln!(image_file, "{} {} {}\n", ir, ig, ib).unwrap();
            }
        }

        println!("Print finished.");
    }
}
