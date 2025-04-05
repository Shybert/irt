mod irt;
use irt::*;
use rand::prelude::*;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
    time::Instant,
};

// fn scene_with_a_lot() {
//     let material_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
//
//     let mut world = vec![Sphere::new(
//         Point::new(0., -1000., 0.),
//         1000.,
//         &material_ground,
//     )];
//
//     for a in -11..11 {
//         for b in -11..11 {
//             let material_choice: f32 = random();
//             let center = Point::new(
//                 a as f32 + 0.9 * random::<f32>(),
//                 0.2,
//                 b as f32 + 0.9 * random::<f32>(),
//             );
//
//             if (center - Point::new(4., 0.2, 0.)).length() <= 0.9 {
//                 continue;
//             }
//
//             if material_choice < 0.8 {
//                 let albedo = Color::random() * Color::random();
//                 let material = Lambertian::new(albedo);
//                 world.push(Sphere::new(center, 0.2, &material));
//             } else if material_choice < 0.95 {
//                 let albedo = Color::random_in_interval(&Interval::new(0.5, 1.));
//                 let fuzz = thread_rng().gen_range(0.5..1.);
//                 let material = Metal::new(albedo, fuzz);
//                 world.push(Sphere::new(center, 0.2, &material));
//             } else {
//                 let material = Dielectric::new(1.5);
//                 world.push(Sphere::new(center, 0.2, &material));
//             }
//         }
//     }
//
//     let material_1 = Dielectric::new(1.5);
//     world.push(Sphere::new(Point::new(0., 1., 0.), 1., &material_1));
//
//     let material_2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
//     world.push(Sphere::new(Point::new(-4., 1., 0.), 1., &material_2));
//
//     let material_3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.);
//     world.push(Sphere::new(Point::new(4., 1., 0.), 1., &material_3));
//
//     let look_from = Point::new(13., 2., 3.);
//     let look_at = Point::new(0., 0., 0.);
//     let up = Vec3::new(0., 1., 0.);
//     let camera = Camera::new(16. / 9., 20., 400, look_from, look_at, up, 100);
//
//     // println!("Building BVH");
//     // let start_time = Instant::now();
//     // let bvh = Node::new(world);
//     // println!(
//     //     "Wall time to build BVH: {:.1} s",
//     //     start_time.elapsed().as_secs_f64()
//     // );
//     //
//     // println!("Rendering scene");
//     // camera.render(&bvh);
// }

fn basic_scene() {
    let white_green_checker = CheckeredTexture::new(
        0.4,
        Box::new(Color::new(0.2, 0.3, 0.1)),
        Box::new(Color::new(0.9, 0.9, 0.9)),
    );
    let material_ground = Lambertian::new(Box::new(white_green_checker));
    let material_center = Lambertian::new(Box::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Dielectric::new(1.5);
    let material_bubble = Dielectric::new(1. / 1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.7);

    let world = vec![
        Sphere::new(Point::new(0., -100.5, -1.), 100., &material_ground),
        Sphere::new(Point::new(0., 0., -1.2), 0.5, &material_center),
        Sphere::new(Point::new(-1., 0., -1.), 0.5, &material_left),
        Sphere::new(Point::new(-1., 0., -1.), 0.4, &material_bubble),
        Sphere::new(Point::new(1., 0., -1.), 0.5, &material_right),
    ];

    // let look_from = Point::new(0., 0., 0.);
    let look_from = Point::new(-2., 2., 1.);
    let look_at = Point::new(0., 0., -1.);
    let up = Vec3::new(0., 1., 0.);

    let camera = Camera::new(16. / 9., 30., 400, look_from, look_at, up, 100);

    let bvh = Bvh::new(world);
    camera.render(&bvh);
}

fn checkered_spheres() {
    let white_green_checker = CheckeredTexture::new(
        0.4,
        Box::new(Color::new(0.2, 0.3, 0.1)),
        Box::new(Color::new(0.9, 0.9, 0.9)),
    );
    let material = Lambertian::new(Box::new(white_green_checker));

    let world = vec![
        Sphere::new(Point::new(0., -10., 0.), 10., &material),
        Sphere::new(Point::new(0., 10., 0.), 10., &material),
    ];

    let look_from = Point::new(13., 2., 3.);
    let look_at = Point::new(0., 0., 0.);
    let up = Vec3::new(0., 1., 0.);
    let camera = Camera::new(16. / 9., 20., 400, look_from, look_at, up, 100);

    let bvh = Bvh::new(world);
    camera.render(&bvh);
}

fn earth() {
    let earth_texture = ImageTexture::new("assets/earthmap.jpg");
    let earth_material = Lambertian::new(Box::new(earth_texture));

    let world = vec![Sphere::new(Point::new(0., 0., 0.), 2., &earth_material)];

    let look_from = Point::new(0., 12., 5.);
    let look_at = Point::new(0., 0., 0.);
    let up = Vec3::new(0., 1., 0.);
    let camera = Camera::new(16. / 9., 20., 400, look_from, look_at, up, 100);

    let bvh = Bvh::new(world);
    camera.render(&bvh);
}

fn noise_scene() {
    let texture = NoiseTexture::new(4.);
    let material = Lambertian::new(Box::new(texture));

    let world = vec![
        Sphere::new(Point::new(0., -1000., 0.), 1000., &material),
        Sphere::new(Point::new(0., 2., 0.), 2., &material),
    ];

    let look_from = Point::new(13., 2., 3.);
    let look_at = Point::new(0., 0., 0.);
    let up = Vec3::new(0., 1., 0.);
    let camera = Camera::new(16. / 9., 20., 400, look_from, look_at, up, 100);

    let bvh = Bvh::new(world);
    camera.render(&bvh);
}

fn parse_triangle<'a>(line: &str, material: &'a dyn Material) -> Triangle<'a> {
    let values: Vec<f32> = line
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    return Triangle::new(
        Point::new(values[0], values[1], values[2]),
        Point::new(values[3], values[4], values[5]),
        Point::new(values[6], values[7], values[8]),
        material,
    );
}

fn read_file<'a>(file_name: &str, material: &'a dyn Material) -> Vec<Triangle<'a>> {
    let file = BufReader::new(File::open(file_name).unwrap());

    return file
        .lines()
        .map(|line| parse_triangle(&line.unwrap(), material))
        .collect();
}

fn scene_robot() {
    let material = Rc::new(Lambertian::new(Box::new(Color::new(0.8, 0.8, 0.))));
    let triangles = read_file("assets/unity.tri", material.as_ref());

    println!("Building BVH");
    let bvh_start_time = Instant::now();
    let mut bvh = Bvh::new(triangles);
    println!(
        "Wall time to build BVH: {:.1} ms",
        bvh_start_time.elapsed().as_millis()
    );

    let sah_start_time = Instant::now();
    println!("Total SAH cost before: {}", bvh.sah2(0));
    bvh.rotate();
    println!("Total SAH cost after: {}", bvh.sah2(0));
    println!(
        "Wall time to compute SAH and rotate: {:.3} microseconds",
        sah_start_time.elapsed().as_micros()
    );

    let look_from = Point::new(-0., 0., -5.);
    let look_at = Point::new(0., 0., -1.);
    let up = Vec3::new(0., 1., 0.);
    let camera = Camera::new(16. / 9., 30., 400, look_from, look_at, up, 100);
    camera.render(&bvh);
}

fn main() {
    println!("Hello, world!");
    let start_time = Instant::now();

    let scene = 5;
    match scene {
        1 => basic_scene(),
        2 => scene_robot(),
        3 => checkered_spheres(),
        4 => earth(),
        5 => noise_scene(),
        _ => basic_scene(),
    }

    println!("Wall time: {:.1} s", start_time.elapsed().as_secs_f64());
}
