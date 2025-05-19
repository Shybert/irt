mod irt;
use irt::*;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
    time::Instant,
};

fn basic_scene() {
    let material_ground = Lambertian::new(Box::new(Color::new(0.8, 0.8, 0.)));
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

    let look_from = Point::new(-2., 2., 1.);
    let look_at = Point::new(0., 0., -1.);
    let up = Vec3::new(0., 1., 0.);

    let camera = Camera::new(
        16. / 9.,
        Degrees(30.),
        400,
        look_from,
        look_at,
        up,
        100,
        Color::new(0.7, 0.8, 1.),
    );

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
    let camera = Camera::new(
        16. / 9.,
        Degrees(20.),
        400,
        look_from,
        look_at,
        up,
        100,
        Color::new(0.7, 0.8, 1.),
    );

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
    let camera = Camera::new(
        16. / 9.,
        Degrees(20.),
        400,
        look_from,
        look_at,
        up,
        100,
        Color::new(0.7, 0.8, 1.),
    );

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
    let camera = Camera::new(
        16. / 9.,
        Degrees(20.),
        400,
        look_from,
        look_at,
        up,
        100,
        Color::new(0.7, 0.8, 1.),
    );

    let bvh = Bvh::new(world);
    camera.render(&bvh);
}

fn quads() {
    let left_red = Lambertian::new(Box::new(Color::new(1.0, 0.2, 0.2)));
    let back_green = Lambertian::new(Box::new(Color::new(0.2, 1.0, 0.2)));
    let upper_orange = Lambertian::new(Box::new(Color::new(1.0, 0.5, 0.)));

    let earth_texture = ImageTexture::new("assets/earthmap.jpg");
    let earth_material = Lambertian::new(Box::new(earth_texture));

    let noise_material = Lambertian::new(Box::new(NoiseTexture::new(4.)));

    let world = vec![
        Quad::new(
            Point::new(-3., -2., 5.),
            Vec3::new(0., 0., -4.),
            Vec3::new(0., 4., 0.),
            &left_red,
        ),
        Quad::new(
            Point::new(-2., -2., 0.),
            Vec3::new(4., 0., 0.),
            Vec3::new(0., 4., 0.),
            &back_green,
        ),
        Quad::new(
            Point::new(3., -2., 1.),
            Vec3::new(0., 0., 4.),
            Vec3::new(0., 4., 0.),
            &earth_material,
        ),
        Quad::new(
            Point::new(-2., 3., 1.),
            Vec3::new(4., 0., 0.),
            Vec3::new(0., 0., 4.),
            &upper_orange,
        ),
        Quad::new(
            Point::new(-2., -3., 5.),
            Vec3::new(4., 0., 0.),
            Vec3::new(0., 0., -4.),
            &noise_material,
        ),
    ];

    let look_from = Point::new(0., 0., 9.);
    let look_at = Point::new(0., 0., 0.);
    let up = Vec3::new(0., 1., 0.);
    let camera = Camera::new(
        1.,
        Degrees(80.),
        400,
        look_from,
        look_at,
        up,
        100,
        Color::new(0.7, 0.8, 1.),
    );

    let bvh = Bvh::new(world);
    camera.render(&bvh);
}

fn simple_light() {
    let noise_texture = NoiseTexture::new(4.);
    let noise_material = Lambertian::new(Box::new(noise_texture));

    let light_material = DiffuseLight::new(Box::new(Color::new(4., 4., 4.)));

    let ground = Sphere::new(Point::new(0., -1000., 0.), 1000., &noise_material);
    let ball = Sphere::new(Point::new(0., 2., 0.), 2., &noise_material);
    let quad_light = Quad::new(
        Point::new(3., 1., -2.),
        Vec3::new(2., 0., 0.),
        Vec3::new(0., 2., 0.),
        &light_material,
    );
    let sphere_light = Sphere::new(Point::new(0., 7., 0.), 2., &light_material);
    let world: Vec<&dyn Hittable> = vec![&ground, &ball, &quad_light, &sphere_light];

    let look_from = Point::new(26., 3., 6.);
    let look_at = Point::new(0., 2., 0.);
    let up = Vec3::new(0., 1., 0.);
    let camera = Camera::new(
        16. / 9.,
        Degrees(20.),
        400,
        look_from,
        look_at,
        up,
        100,
        Color::black(),
    );

    let bvh = Bvh::new(world);
    camera.render(&bvh);
}

fn cornell_box() {
    let red = Lambertian::new(Box::new(Color::new(0.65, 0.05, 0.05)));
    let green = Lambertian::new(Box::new(Color::new(0.12, 0.45, 0.15)));
    let white = Lambertian::new(Box::new(Color::new(0.73, 0.73, 0.73)));
    let light = DiffuseLight::new(Box::new(Color::new(15., 15., 15.)));

    let outer_box = Bvh::new(vec![
        Quad::new(
            Point::new(555., 0., 0.),
            Vec3::new(0., 555., 0.),
            Vec3::new(0., 0., 555.),
            &green,
        ),
        Quad::new(
            Point::new(0., 0., 0.),
            Vec3::new(0., 555., 0.),
            Vec3::new(0., 0., 555.),
            &red,
        ),
        Quad::new(
            Point::new(343., 554., 332.),
            Vec3::new(-130., 0., 0.),
            Vec3::new(0., 0., -105.),
            &light,
        ),
        Quad::new(
            Point::new(0., 0., 0.),
            Vec3::new(555., 0., 0.),
            Vec3::new(0., 0., 555.),
            &white,
        ),
        Quad::new(
            Point::new(0., 555., 0.),
            Vec3::new(555., 0., 0.),
            Vec3::new(0., 0., 555.),
            &white,
        ),
        Quad::new(
            Point::new(0., 0., 555.),
            Vec3::new(555., 0., 0.),
            Vec3::new(0., 555., 0.),
            &white,
        ),
    ]);
    let outer_box_instance = BVHInstance::new(&outer_box, Matrix::identity());

    let cube = Bvh::new(Quad::cube(&white));
    let box1_instance = BVHInstance::new(
        &cube,
        Matrix::identity()
            .scale(165., 330., 165.)
            .translate(150., 0., 370.)
            .rotate_y(Degrees(15.)),
    );
    let box2_instance = BVHInstance::new(
        &cube,
        Matrix::identity()
            .scale(165., 165., 165.)
            .translate(160., 0., 65.)
            .rotate_y(Degrees(-18.)),
    );

    let look_from = Point::new(278., 278., -800.);
    let look_at = Point::new(278., 278., 0.);
    let up = Vec3::new(0., 1., 0.);
    let camera = Camera::new(
        1.,
        Degrees(40.),
        600,
        look_from,
        look_at,
        up,
        200,
        Color::black(),
    );

    camera.render(&Bvh::new(vec![
        outer_box_instance,
        box1_instance,
        box2_instance,
    ]));
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
    let camera = Camera::new(
        16. / 9.,
        Degrees(30.),
        400,
        look_from,
        look_at,
        up,
        100,
        Color::new(0.7, 0.8, 1.),
    );
    camera.render(&bvh);
}

fn armadillos() {
    let material = Rc::new(Lambertian::new(Box::new(Color::new(0.8, 0.8, 0.))));
    let triangles = read_file("assets/armadillo.tri", material.as_ref());

    let look_from = Point::new(0., 0., -8.);
    let look_at = Point::new(0., 0., -1.);
    let up = Vec3::new(0., 1., 0.);
    let camera = Camera::new(
        16. / 9.,
        Degrees(30.),
        400,
        look_from,
        look_at,
        up,
        100,
        Color::new(0.7, 0.8, 1.),
    );

    let bvh = Bvh::new(triangles);
    let bvh_instance = BVHInstance::new(
        &bvh,
        Matrix::identity().scale(0.3, 2., 1.).translate(-2., 0., 0.),
    );
    let bvh_instance2 = BVHInstance::new(&bvh, Matrix::identity().translate(2., 0., 0.));
    let bvh_instance3 = BVHInstance::new(&bvh, Matrix::identity().rotate_y(Degrees(-45.)));
    // let tlas = Bvh::new(vec![bvh_instance, bvh_instance2, bvh_instance3]);
    // let tlas = Bvh::new(vec![bvh_instance3]);
    let tlas = Bvh::new(vec![bvh_instance, bvh_instance2, bvh_instance3]);
    camera.render(&tlas);
    // camera.render(&bvh_instance2);
}

fn main() {
    println!("Hello, world!");
    let start_time = Instant::now();

    let scene = 8;
    match scene {
        1 => basic_scene(),
        2 => scene_robot(),
        3 => checkered_spheres(),
        4 => earth(),
        5 => noise_scene(),
        6 => quads(),
        7 => simple_light(),
        8 => cornell_box(),
        9 => armadillos(),
        _ => basic_scene(),
    }

    println!("Wall time: {:.1} s", start_time.elapsed().as_secs_f64());
}
