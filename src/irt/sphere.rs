use std::rc::Rc;

use crate::{Aabb, Hit, Hittable, Interval, Material, Point, Ray, Vec3};

#[derive(Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Rc<dyn Material>,
    aabb: Aabb,
}
impl Sphere {
    pub fn new(center: Point, radius: f32, material: Rc<dyn Material>) -> Self {
        let radius2 = radius.max(0.);
        let radius_vector = Vec3::new(radius2, radius2, radius2);
        let aabb = Aabb::new(center - radius_vector, center + radius_vector);

        return Self {
            center,
            radius: radius2,
            material,
            aabb,
        };
    }
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_interval: &mut Interval) -> Option<Hit> {
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
        t_interval.max = t;
        let point = ray.at(t);
        return Some(Hit::new(
            ray,
            point,
            (point - self.center) / self.radius,
            t,
            self.material.as_ref(),
        ));
    }

    fn aabb(&self) -> &Aabb {
        return &self.aabb;
    }
}
