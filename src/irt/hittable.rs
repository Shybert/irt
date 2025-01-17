use crate::{Interval, Point, Ray, Vec3};

pub struct Hit {
    pub point: Point,
    pub normal: Vec3,
    pub t: f32,
}
impl Hit {
    pub fn new(point: Point, normal: Vec3, t: f32) -> Self {
        return Self { point, normal, t };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_interval: &Interval) -> Option<Hit>;
}
impl<T: Hittable> Hittable for &[T] {
    fn hit(&self, ray: &Ray, t_interval: &Interval) -> Option<Hit> {
        return self
            .iter()
            .filter_map(|sphere| sphere.hit(ray, t_interval))
            .min_by(|x, y| x.t.total_cmp(&y.t));
    }
}
