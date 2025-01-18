mod irt;
use irt::*;

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

fn main() {
    println!("Hello, world!");

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Metal::new(Color::new(0.8, 0.8, 0.8));
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2));

    let world = vec![
        Sphere::new(Point::new(0., -100.5, -1.), 100., &material_ground),
        Sphere::new(Point::new(0., 0., -1.), 0.5, &material_center),
        Sphere::new(Point::new(-1., 0., -1.), 0.5, &material_left),
        Sphere::new(Point::new(1., 0., -1.), 0.5, &material_right),
    ];

    let camera = Camera::new(16. / 9., 90., 400, 100);
    camera.render(&world.as_slice());
}
