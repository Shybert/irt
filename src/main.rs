mod irt;
use irt::*;
use rand::prelude::*;

use std::{rc::Rc, time::Instant};

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

fn scene_with_a_lot() {
    let material_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));

    let mut world = vec![Sphere::new(
        Point::new(0., -1000., 0.),
        1000.,
        Rc::new(material_ground),
    )];

    for a in -11..11 {
        for b in -11..11 {
            let material_choice: f32 = random();
            let center = Point::new(
                a as f32 + 0.9 * random::<f32>(),
                0.2,
                b as f32 + 0.9 * random::<f32>(),
            );

            if (center - Point::new(4., 0.2, 0.)).length() <= 0.9 {
                continue;
            }

            if material_choice < 0.8 {
                let albedo = Color::random() * Color::random();
                let material = Lambertian::new(albedo);
                world.push(Sphere::new(center, 0.2, Rc::new(material)));
            } else if material_choice < 0.95 {
                let albedo = Color::random_in_interval(&Interval::new(0.5, 1.));
                let fuzz = thread_rng().gen_range(0.5..1.);
                let material = Metal::new(albedo, fuzz);
                world.push(Sphere::new(center, 0.2, Rc::new(material)));
            } else {
                let material = Dielectric::new(1.5);
                world.push(Sphere::new(center, 0.2, Rc::new(material)));
            }
        }
    }

    let material_1 = Dielectric::new(1.5);
    world.push(Sphere::new(Point::new(0., 1., 0.), 1., Rc::new(material_1)));

    let material_2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.push(Sphere::new(
        Point::new(-4., 1., 0.),
        1.,
        Rc::new(material_2),
    ));

    let material_3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.);
    world.push(Sphere::new(Point::new(4., 1., 0.), 1., Rc::new(material_3)));

    let look_from = Point::new(13., 2., 3.);
    let look_at = Point::new(0., 0., 0.);
    let up = Vec3::new(0., 1., 0.);
    let camera = Camera::new(16. / 9., 20., 400, look_from, look_at, up, 100);

    println!("Building BVH");
    let bvh = Node::new(world);

    println!("Rendering scene");
    camera.render(&bvh);
}

fn basic_scene() {
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_bubble = Dielectric::new(1. / 1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.7);

    let world = vec![
        Sphere::new(Point::new(0., -100.5, -1.), 100., Rc::new(material_ground)),
        Sphere::new(Point::new(0., 0., -1.2), 0.5, Rc::new(material_center)),
        Sphere::new(Point::new(-1., 0., -1.), 0.5, Rc::new(material_left)),
        Sphere::new(Point::new(-1., 0., -1.), 0.4, Rc::new(material_bubble)),
        Sphere::new(Point::new(1., 0., -1.), 0.5, Rc::new(material_right)),
    ];

    // let look_from = Point::new(0., 0., 0.);
    let look_from = Point::new(-2., 2., 1.);
    let look_at = Point::new(0., 0., -1.);
    let up = Vec3::new(0., 1., 0.);

    let camera = Camera::new(16. / 9., 30., 400, look_from, look_at, up, 100);

    let bvh = Node::new(world);
    println!("bvh, {:?}", bvh);
    camera.render(&bvh);
}

fn main() {
    println!("Hello, world!");
    let start_time = Instant::now();

    let scene = 2;
    match scene {
        1 => basic_scene(),
        2 => scene_with_a_lot(),
        _ => basic_scene(),
    }

    println!("Wall time: {:.1} s", start_time.elapsed().as_secs_f64());
}
