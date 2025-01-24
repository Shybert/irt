use crate::{Aabb, Interval, Material, Point, Ray, Vec3};

#[derive(Debug)]
pub struct Hit<'a> {
    pub point: Point,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: &'a dyn Material,
}
impl<'a> Hit<'a> {
    pub fn new(
        ray: &Ray,
        point: Point,
        outward_normal: Vec3,
        t: f32,
        material: &'a dyn Material,
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
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_interval: &Interval) -> Option<Hit>;

    fn aabb(&self) -> &Aabb;
}

pub struct HittableList<T: Hittable> {
    objects: Vec<T>,
    aabb: Aabb,
}
impl<T: Hittable> HittableList<T> {
    pub fn new(objects: Vec<T>) -> Self {
        let aabb = objects
            .iter()
            .fold(Aabb::empty(), |aabb, object| aabb.expand(object.aabb()));

        return Self { objects, aabb };
    }
}
impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, ray: &Ray, t_interval: &Interval) -> Option<Hit> {
        return self
            .objects
            .iter()
            .filter_map(|object| object.hit(ray, t_interval))
            .min_by(|x, y| x.t.total_cmp(&y.t));
    }

    fn aabb(&self) -> &Aabb {
        return &self.aabb;
    }
}
