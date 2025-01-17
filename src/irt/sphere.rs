use crate::{Hit, Hittable, Interval, Point, Ray};

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
}
impl Sphere {
    pub fn new(center: Point, radius: f32) -> Self {
        return Self {
            center,
            radius: radius.max(0.),
        };
    }
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_interval: &Interval) -> Option<Hit> {
        let oc = self.center - ray.origin;
        let a = ray.direction.magnitude().powi(2);
        let h = ray.direction.dot(&oc);
        let c = oc.magnitude().powi(2) - self.radius.powi(2);

        let discriminant = h.powi(2) - a * c;
        if discriminant < 0. {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (h - discriminant_sqrt) / a;
        if !t_interval.surrounds(root) {
            root = (h + discriminant_sqrt) / a;
            if !t_interval.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        return Some(Hit::new(point, (point - self.center) / self.radius, t));
    }
}
