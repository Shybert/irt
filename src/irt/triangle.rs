use crate::{Aabb, Hit, Hittable, Interval, Material, Point, Ray};

#[derive(Debug)]
pub struct Triangle<'a> {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub centroid: Point,
    aabb: Aabb,
    pub material: &'a dyn Material,
}
impl<'a> Triangle<'a> {
    pub fn new(a: Point, b: Point, c: Point, material: &'a dyn Material) -> Self {
        let centroid = (a + b + c) * (1. / 3.);
        let aabb = Aabb::new(a.min(&b.min(&c)), a.max(&b.max(&c)));

        return Triangle {
            a,
            b,
            c,
            centroid,
            aabb,
            material,
        };
    }
}
impl Hittable for Triangle<'_> {
    fn aabb(&self) -> Aabb {
        return self.aabb;
    }

    fn centroid(&self) -> Point {
        return self.centroid;
    }

    fn hit(&self, ray: &Ray, t_interval: &mut Interval) -> Option<Hit> {
        let edge_1 = self.b - self.a;
        let edge_2 = self.c - self.a;

        let ray_cross_edge_2 = ray.direction.cross(&edge_2);
        let det = edge_1.dot(&ray_cross_edge_2);

        // Parallell ray
        if det > -0.0001 && det < 0.0001 {
            return None;
        }

        let inverse_det = 1. / det;
        let s = ray.origin - self.a;
        let u = inverse_det * s.dot(&ray_cross_edge_2);
        if !(0. ..=1.).contains(&u) {
            return None;
        }

        let s_cross_edge_1 = s.cross(&edge_1);
        let v = inverse_det * ray.direction.dot(&s_cross_edge_1);
        if v < 0. || u + v > 1. {
            return None;
        }

        let t = inverse_det * edge_2.dot(&s_cross_edge_1);
        if !t_interval.surrounds(t) {
            return None;
        }

        t_interval.max = t;
        return Some(Hit::new(
            ray,
            ray.origin + ray.direction * t,
            edge_2.cross(&edge_1),
            t,
            self.material,
            0.,
            0.,
        ));
    }
}
