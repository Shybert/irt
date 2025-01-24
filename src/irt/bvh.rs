use std::rc::Rc;

use crate::{Aabb, Hit, Hittable, Interval, Ray, Sphere};

#[derive(Debug)]
enum NodeOrSpheres {
    Node(Rc<Node>),
    Spheres(Vec<Sphere>),
}

#[derive(Debug)]
pub struct Node {
    aabb: Aabb,
    left: NodeOrSpheres,
    right: NodeOrSpheres,
}
impl Node {
    pub fn new(spheres: Vec<Sphere>) -> Self {
        let mut aabb = Aabb::empty();
        spheres
            .iter()
            .for_each(|sphere| aabb = aabb.expand(sphere.aabb()));

        let extent = aabb.extent();
        let longest_axis = extent.longest_axis();
        let split_position = aabb.min[&longest_axis] + extent[&longest_axis] * 0.5;

        let (left_spheres, right_spheres): (Vec<Sphere>, Vec<Sphere>) = spheres
            .into_iter()
            .partition(|sphere| sphere.center[&longest_axis] < split_position);

        let left = match left_spheres.len() <= 2 {
            true => NodeOrSpheres::Spheres(left_spheres),
            false => NodeOrSpheres::Node(Rc::new(Self::new(left_spheres))),
        };
        let right = match right_spheres.len() <= 2 {
            true => NodeOrSpheres::Spheres(right_spheres),
            false => NodeOrSpheres::Node(Rc::new(Self::new(right_spheres))),
        };

        return Self { aabb, left, right };
    }
}
fn hit_a_sphere<'a>(spheres: &'a [Sphere], ray: &Ray, t_interval: &Interval) -> Option<Hit<'a>> {
    return spheres
        .iter()
        .filter_map(|object| object.hit(ray, t_interval))
        .min_by(|x, y| x.t.total_cmp(&y.t));
}
impl Hittable for Node {
    fn aabb(&self) -> &Aabb {
        return &self.aabb;
    }

    fn hit(&self, ray: &Ray, t_interval: &Interval) -> Option<Hit> {
        if !self.aabb().hit(ray) {
            return None;
        }

        let hit_left = match &self.left {
            NodeOrSpheres::Node(node) => node.hit(ray, t_interval),
            NodeOrSpheres::Spheres(spheres) => hit_a_sphere(spheres, ray, t_interval),
        };

        let hit_right = match &self.right {
            NodeOrSpheres::Node(node) => node.hit(ray, t_interval),
            NodeOrSpheres::Spheres(spheres) => hit_a_sphere(spheres, ray, t_interval),
        };

        if hit_left.is_none() && hit_right.is_none() {
            return None;
        } else if hit_left.is_some() && hit_right.is_none() {
            return hit_left;
        } else if hit_left.is_none() && hit_right.is_some() {
            return hit_right;
        } else {
            let left_t = hit_left.as_ref().unwrap().t;
            let right_t = hit_right.as_ref().unwrap().t;
            if left_t <= right_t {
                return hit_left;
            } else {
                return hit_right;
            }
        }
    }
}
