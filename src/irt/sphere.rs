use std::f32::consts::PI;

use crate::irt::{Aabb, Hit, Hittable, Interval, Material, Point, Ray, Vec3};

#[derive(Debug)]
pub struct Sphere<'a> {
    pub center: Point,
    pub radius: f32,
    pub material: &'a dyn Material,
    aabb: Aabb,
}
impl<'a> Sphere<'a> {
    pub fn new(center: Point, radius: f32, material: &'a dyn Material) -> Self {
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

    pub fn uv_at(point: &Point) -> (f32, f32) {
        let theta = (-point.y).acos();
        let phi = (-point.z).atan2(point.x) + PI;

        let u = phi / (2. * PI);
        let v = theta / PI;
        return (u, v);
    }
}
impl Hittable for Sphere<'_> {
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
        let outward_normal = (point - self.center) / self.radius;
        let (u, v) = Sphere::uv_at(&outward_normal.into());
        return Some(Hit::new(ray, point, outward_normal, t, self.material, u, v));
    }

    fn aabb(&self) -> Aabb {
        return self.aabb;
    }
}
