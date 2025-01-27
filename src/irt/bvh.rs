use std::rc::Rc;

use strum::IntoEnumIterator;

use crate::{Aabb, Axis, Hit, Hittable, Interval, Ray, Triangle};

#[derive(Debug)]
enum NodeOrSpheres<'a> {
    Node(Rc<Node<'a>>),
    Spheres(Vec<Triangle<'a>>),
}

#[derive(Debug)]
pub struct Node<'a> {
    aabb: Aabb,
    left: NodeOrSpheres<'a>,
    right: NodeOrSpheres<'a>,
}
impl<'a> Node<'a> {
    pub fn sah(triangles: &[Triangle], axis: &Axis, position: f32) -> f32 {
        let mut left_box = Aabb::empty();
        let mut left_count = 0.;
        let mut right_box = Aabb::empty();
        let mut right_count = 0.;

        for triangle in triangles {
            if triangle.centroid[axis] < position {
                left_count += 1.;
                left_box.expand_to_point(&triangle.a);
                left_box.expand_to_point(&triangle.b);
                left_box.expand_to_point(&triangle.c);
            } else {
                right_count += 1.;
                right_box.expand_to_point(&triangle.a);
                right_box.expand_to_point(&triangle.b);
                right_box.expand_to_point(&triangle.c);
            }
        }

        let cost = left_count * left_box.area() + right_count * right_box.area();
        return if cost > 0. { cost } else { f32::INFINITY };
    }

    pub fn new(spheres: Vec<Triangle<'a>>) -> Node<'a> {
        let mut aabb = Aabb::empty();
        spheres
            .iter()
            .for_each(|sphere| aabb = aabb.expand(sphere.aabb()));
        let current_cost = aabb.area() * spheres.len() as f32;

        let mut best_axis = Axis::X;
        let mut best_cost = f32::INFINITY;
        let mut best_position = 0.;
        for axis in Axis::iter() {
            for triangle in &spheres {
                let candidate_position = triangle.centroid[&axis];
                let cost = Node::sah(&spheres, &axis, candidate_position);
                if cost < best_cost {
                    best_axis = axis;
                    best_cost = cost;
                    best_position = candidate_position;
                }
            }
        }

        let (left_spheres, right_spheres): (Vec<Triangle>, Vec<Triangle>) = spheres
            .into_iter()
            .partition(|triangle| triangle.centroid[&best_axis] < best_position);

        if best_cost >= current_cost {
            return Self {
                aabb,
                left: NodeOrSpheres::Spheres(left_spheres),
                right: NodeOrSpheres::Spheres(right_spheres),
            };
        }

        return Self {
            aabb,
            left: NodeOrSpheres::Node(Rc::new(Self::new(left_spheres))),
            right: NodeOrSpheres::Node(Rc::new(Self::new(right_spheres))),
        };
    }
}
fn hit_a_sphere<'a>(spheres: &'a [Triangle], ray: &Ray, t_interval: &Interval) -> Option<Hit<'a>> {
    return spheres
        .iter()
        .filter_map(|object| object.hit(ray, t_interval))
        .min_by(|x, y| x.t.total_cmp(&y.t));
}
impl Hittable for Node<'_> {
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
