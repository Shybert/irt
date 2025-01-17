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

    let camera = Camera::new(16. / 9., 400, 100);
    let world = vec![
        Sphere::new(Point::new(0., 0., -1.), 0.5),
        Sphere::new(Point::new(0., -100.5, -1.), 100.),
    ];

    camera.render(&world.as_slice());
}
