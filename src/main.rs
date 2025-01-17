mod irt;
use irt::*;

use std::fs::File;
use std::io::Write;

fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
    let potential_hit = world.hit(ray, &Interval::new(0., f32::INFINITY));
    if let Some(hit) = potential_hit {
        return 0.5 * Color::new(hit.normal.x + 1., hit.normal.y + 1., hit.normal.z + 1.);
    }

    let a = 0.5 * (ray.direction.normalize().y + 1.);
    return (1. - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.0);
}

struct Triangle {
    v0: Point,
    v1: Point,
    v2: Point,
    centroid: Point,
}
impl Triangle {
    pub fn new(v0: Point, v1: Point, v2: Point, centroid: Point) -> Self {
        return Triangle {
            v0,
            v1,
            v2,
            centroid,
        };
    }
}

fn print_image() {
    let aspect_ratio = 16. / 9.;
    let image_width = 400;

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

    let mut image_file = File::create("image.ppm").expect("Could not create image file.");
    writeln!(image_file, "P3\n{} {}\n255\n", image_width, image_height).unwrap();

    for j in 0..image_height {
        println!("Lines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let spheres = vec![
                Sphere::new(Point::new(0., 0., -1.), 0.5),
                Sphere::new(Point::new(0., -100.5, -1.), 100.),
            ];

            let color = ray_color(&ray, &spheres.as_slice());

            let ir = (255.999 * color.r) as u32;
            let ig = (255.999 * color.g) as u32;
            let ib = (255.999 * color.b) as u32;

            writeln!(image_file, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }

    println!("Print finished.");
}

fn main() {
    println!("Hello, world!");
    // let random_point = Vec3::random();
    // println!("{:?}", random_point * 9. - Vec3::new(5., 5., 5.));
    print_image();
}
