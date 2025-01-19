use crate::{Hit, Hittable, Interval, Material, Point, Ray};

pub struct Sphere<'a> {
    pub center: Point,
    pub radius: f32,
    pub material: &'a dyn Material,
}
impl<'a> Sphere<'a> {
    pub fn new(center: Point, radius: f32, material: &'a dyn Material) -> Self {
        return Self {
            center,
            radius: radius.max(0.),
            material,
        };
    }
}
impl Hittable for Sphere<'_> {
    fn hit(&self, ray: &Ray, t_interval: &Interval) -> Option<Hit> {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.length_squared() - self.radius.powi(2);

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
        return Some(Hit::new(
            ray,
            point,
            (point - self.center) / self.radius,
            t,
            self.material,
        ));
    }
}
