use crate::irt::{Aabb, Interval, Material, Point, Ray, Vec3};

/// A hit record. The normal is guaranteed to be of unit length.
#[derive(Debug)]
pub struct Hit<'a> {
    pub point: Point,
    /// Guaranteed to be of unit length.
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: &'a dyn Material,
    pub u: f32,
    pub v: f32,
}
impl<'a> Hit<'a> {
    /// Create a new hit record. The normal must be of unit length.
    pub fn new(
        ray: &Ray,
        point: Point,
        outward_normal: Vec3,
        t: f32,
        material: &'a dyn Material,
        u: f32,
        v: f32,
    ) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.;
        let normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };

        return Self {
            point,
            normal,
            t,
            front_face,
            material,
            u,
            v,
        };
    }
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_interval: &mut Interval) -> Option<Hit>;

    fn bounds(&self) -> Aabb;

    fn centroid(&self) -> Point {
        let bounds = self.bounds();
        return bounds.min + bounds.extent() * 0.5;
    }
}
impl<T: Hittable> Hittable for [T] {
    fn hit(&self, ray: &Ray, t_interval: &mut Interval) -> Option<Hit> {
        return self
            .iter()
            .filter_map(|hittable| hittable.hit(ray, t_interval))
            .min_by(|x, y| x.t.total_cmp(&y.t));
    }

    fn bounds(&self) -> Aabb {
        return self.iter().fold(Aabb::empty(), |bounds, hittable| {
            bounds.expand(&hittable.bounds())
        });
    }
}

// This might be hacky? But it allows using trait objects
// with BVHs while also allowing static dispatch for worlds
// containing only a single primitive. This might be pre-mature
// optimization, but it is cool!
impl Hittable for &dyn Hittable {
    fn hit(&self, ray: &Ray, t_interval: &mut Interval) -> Option<Hit> {
        return (*self).hit(ray, t_interval);
    }

    fn bounds(&self) -> Aabb {
        return (*self).bounds();
    }

    fn centroid(&self) -> Point {
        return (*self).centroid();
    }
}
