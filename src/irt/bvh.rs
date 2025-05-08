use std::ops::Range;

use itertools::partition;

use crate::irt::{Aabb, Axis, Hit, Hittable, Interval, Matrix, Ray};

struct Split {
    axis: Axis,
    position: f32,
    cost: f32,
}
impl Split {
    fn new(axis: Axis, position: f32, cost: f32) -> Self {
        return Self {
            axis,
            position,
            cost,
        };
    }
}

pub struct Bvh<T: Hittable> {
    hittables: Vec<T>,
    nodes: Vec<BvhNode>,
}
impl<T: Hittable> Bvh<T> {
    fn children(&self, node_index: usize) -> Option<(&BvhNode, &BvhNode)> {
        let node = &self.nodes[node_index];

        if node.is_leaf() {
            return None;
        }

        return Some((
            &self.nodes[node.left_first],
            &self.nodes[node.left_first + 1],
        ));
    }

    fn recompute_bounds(&mut self, node_index: usize) {
        let children = self.children(node_index);

        let node = &self.nodes[node_index];
        self.nodes[node_index].bounds = match children {
            None => self.hittables[node.hittable_range()].bounds(),
            Some((left_child, right_child)) => left_child.bounds.expand(&right_child.bounds),
        };
    }

    pub fn rotate(&mut self) {
        let right_child_index = self.nodes[0].left_first + 1;
        let right_child_left_child_index = self.nodes[right_child_index].left_first;

        let left_child_index = self.nodes[0].left_first;

        self.nodes
            .swap(left_child_index, right_child_left_child_index);

        self.recompute_bounds(right_child_index);
        self.recompute_bounds(0);
    }

    pub fn sah2(&self, node_index: usize) -> f32 {
        const C_I: f32 = 1.2;
        const C_T: f32 = 1.;

        let node = &self.nodes[node_index];
        return match self.children(node_index) {
            Some((left_child, right_child)) => {
                C_T + (left_child.bounds.area() * self.sah2(node.left_first)
                    + (right_child.bounds.area() * self.sah2(node.left_first + 1)))
                    / (node.bounds.area())
            }
            None => C_T + C_I * node.hittable_count as f32,
        };
    }

    fn sah(hittables: &[T], axis: &Axis, position: f32) -> f32 {
        let mut left_box = Aabb::empty();
        let mut left_count = 0.;
        let mut right_box = Aabb::empty();
        let mut right_count = 0.;

        for hittable in hittables {
            if hittable.centroid()[axis] < position {
                left_count += 1.;
                left_box = left_box.expand(&hittable.bounds());
            } else {
                right_count += 1.;
                right_box = right_box.expand(&hittable.bounds());
            }
        }

        let cost = left_count * left_box.area() + right_count * right_box.area();
        return if cost > 0. { cost } else { f32::INFINITY };
    }

    fn best_split(hittables: &[T]) -> Split {
        let mut best_split = Split::new(Axis::X, 0., f32::INFINITY);

        for axis in Axis::iter() {
            let mut bound = Interval::empty();
            for hittable in hittables {
                bound.min = bound.min.min(hittable.centroid()[&axis]);
                bound.max = bound.max.max(hittable.centroid()[&axis]);
            }
            if bound.min == bound.max {
                continue;
            }

            let num_intervals = 4;
            let scale = bound.size() / num_intervals as f32;
            for i in 1..num_intervals {
                let candidate_position = bound.min + i as f32 * scale;
                let cost = Self::sah(hittables, &axis, candidate_position);
                if cost <= best_split.cost {
                    best_split = Split::new(axis, candidate_position, cost);
                }
            }
        }

        return best_split;
    }

    fn subdivide(&mut self, node_index: usize) {
        let node = &self.nodes[node_index];

        let best_split = Self::best_split(&self.hittables[node.hittable_range()]);
        let parent_cost = node.bounds.area() * node.hittable_count as f32;
        if best_split.cost >= parent_cost {
            return;
        }

        let split_index = partition(&mut self.hittables[node.hittable_range()], |hittable| {
            hittable.centroid()[&best_split.axis] <= best_split.position
        });
        if split_index == 0 || split_index == node.hittable_count {
            return;
        }

        let left_node = BvhNode::new(self, node.left_first, split_index);
        let right_node = BvhNode::new(
            self,
            node.left_first + split_index,
            node.hittable_count - split_index,
        );

        let left_index = self.nodes.len();
        self.nodes.push(left_node);
        self.nodes.push(right_node);
        self.subdivide(left_index);
        self.subdivide(left_index + 1);

        self.nodes[node_index].left_first = left_index;
        self.nodes[node_index].hittable_count = 0;

        return;
    }

    pub fn new(hittables: Vec<T>) -> Self {
        let nodes = Vec::with_capacity(hittables.len() * 2 - 1);

        let mut bvh = Self { hittables, nodes };
        let root_node = BvhNode::new(&bvh, 0, bvh.hittables.len());

        bvh.nodes.push(root_node);
        bvh.subdivide(0);
        bvh.nodes.shrink_to_fit();

        return bvh;
    }

    fn intersect(&self, ray: &Ray, t_interval: &mut Interval, node_index: usize) -> Option<Hit> {
        let node = &self.nodes[node_index];
        if !node.bounds.hit(ray, t_interval) {
            return None;
        }

        if node.is_leaf() {
            return self.hittables[node.hittable_range()].hit(ray, t_interval);
        } else {
            let hit_left = self.intersect(ray, t_interval, node.left_first);
            let hit_right = self.intersect(ray, t_interval, node.left_first + 1);

            return match (&hit_left, &hit_right) {
                (Some(left), Some(right)) => {
                    if left.t <= right.t {
                        hit_left
                    } else {
                        hit_right
                    }
                }
                _ => hit_left.or(hit_right),
            };
        }
    }
}
#[derive(Debug)]
struct BvhNode {
    bounds: Aabb,
    left_first: usize,
    hittable_count: usize,
}
impl BvhNode {
    fn new<T: Hittable>(bvh: &Bvh<T>, left_first: usize, hittable_count: usize) -> Self {
        return Self {
            bounds: bvh.hittables[left_first..left_first + hittable_count].bounds(),
            left_first,
            hittable_count,
        };
    }

    fn hittable_range(&self) -> Range<usize> {
        return self.left_first..self.left_first + self.hittable_count;
    }

    /// Returns whether the node is a leaf, i.e. whether it contains hittables
    fn is_leaf(&self) -> bool {
        return self.hittable_count > 0;
    }
}

impl<T: Hittable> Hittable for Bvh<T> {
    fn bounds(&self) -> Aabb {
        return self.nodes[0].bounds;
    }

    fn hit(&self, ray: &Ray, t_interval: &mut Interval) -> Option<Hit> {
        return self.intersect(ray, t_interval, 0);
    }
}

pub struct BVHInstance<'a, T: Hittable> {
    bvh: &'a Bvh<T>,
    inverse_transform: Matrix,
}
impl<'a, T: Hittable> BVHInstance<'a, T> {
    pub fn new(bvh: &'a Bvh<T>, transform: Matrix) -> Self {
        return Self {
            bvh,
            inverse_transform: transform.inverse(),
        };
    }
}

impl<T: Hittable> Hittable for BVHInstance<'_, T> {
    fn bounds(&self) -> Aabb {
        return self.bvh.bounds();
    }
    }

    fn hit(&self, ray: &Ray, t_interval: &mut Interval) -> Option<Hit> {
        return self.bvh.hit(&(self.inverse_transform * *ray), t_interval);
    }
}
