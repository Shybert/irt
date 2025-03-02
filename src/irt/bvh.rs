use std::ops::Range;

use itertools::partition;

use crate::{Aabb, Axis, Hit, Hittable, Interval, Ray, Triangle};

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

pub struct Bvh<'a> {
    triangles: Vec<Triangle<'a>>,
    nodes: Vec<BvhNode>,
}
impl<'a> Bvh<'a> {
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

    fn recompute_aabb(&mut self, node_index: usize) {
        let children = self.children(node_index);

        let node = &self.nodes[node_index];
        self.nodes[node_index].aabb = match children {
            None => self.triangles[node.triangle_range()].aabb(),
            Some((left_child, right_child)) => left_child.aabb.expand(&right_child.aabb),
        };
    }

    pub fn rotate(&mut self) {
        let right_child_index = self.nodes[0].left_first + 1;
        let right_child_left_child_index = self.nodes[right_child_index].left_first;

        let left_child_index = self.nodes[0].left_first;

        self.nodes
            .swap(left_child_index, right_child_left_child_index);

        self.recompute_aabb(right_child_index);
        self.recompute_aabb(0);
    }

    pub fn rotation_can_improve(&mut self, node_index: usize, costs: &mut [f32]) {
        // println!("yes");
        let node = &self.nodes[node_index];
        if node.is_leaf() {
            return;
        }
        let right_child_index = node.left_first + 1;
        let right_child = &self.nodes[right_child_index];
        if right_child.is_leaf() {
            return;
        }

        let left_child_index = node.left_first;
        let left_child = &self.nodes[left_child_index];
        let right_child_right_child_index = right_child.left_first + 1;
        let right_child_right_child = &self.nodes[right_child_right_child_index];
        let right_new_aabb = left_child.aabb.expand(&right_child_right_child.aabb);

        let surface_left = left_child.aabb.area();
        let surface_right = right_child_right_child.aabb.area();
        let cost_left = costs[node.left_first];
        let cost_right = costs[right_child_right_child_index];
        let right_child_new_cost = 1.
            + ((surface_left * cost_left) + (surface_right * cost_right)) / right_child.aabb.area();

        let right_child_left_child_index = right_child.left_first + 1;
        let right_child_left_child = &self.nodes[right_child_left_child_index];

        let surface_left_2 = right_child_left_child.aabb.area();
        let cost_left_2 = costs[right_child_left_child_index];
        let surface_right_2 = right_new_aabb.area();
        // let cost_right_2 = costs[right_child_index];
        let new_cost = 1.
            + ((surface_left_2 * cost_left_2) + (surface_right_2 * right_child_new_cost))
                / node.aabb.area();

        if new_cost < costs[node_index] {
            // println!("cheaper!!");
            // let node_new_aabb = right_child_left_child.aabb.expand(&right_child.aabb);
            self.nodes
                .swap(left_child_index, right_child_left_child_index);

            // right_child.aabb = right_new_aabb;
            // self.nodes[node_index].aabb = node_new_aabb;
            self.nodes[right_child_index].aabb = right_new_aabb;
            // self.recompute_aabb(right_child_index);
            self.recompute_aabb(node_index);
            costs[node_index] = new_cost;
            costs[right_child_index] = right_child_new_cost;
        }
    }

    pub fn sah4(&mut self) -> f32 {
        let mut costs = vec![0.; self.nodes.len()];
        self.sah_recursive(0, &mut costs);
        return costs[0];
    }

    pub fn sah_recursive(&mut self, node_index: usize, costs: &mut [f32]) -> f32 {
        const C_I: f32 = 1.2;
        const C_T: f32 = 1.;

        let node = &self.nodes[node_index];
        costs[node_index] = match self.children(node_index) {
            Some((left_child, right_child)) => {
                let surface_node = node.aabb.area();
                let surface_left = left_child.aabb.area();
                let surface_right = right_child.aabb.area();

                let left_index = node.left_first;
                let right_index = node.left_first + 1;
                let cost_left = self.sah_recursive(left_index, costs);
                let cost_right = self.sah_recursive(right_index, costs);

                C_T + ((surface_left * cost_left + surface_right * cost_right) / surface_node)
            }
            None => C_T + C_I * node.triangle_count as f32,
        };
        self.rotation_can_improve(node_index, costs);
        return costs[node_index];
    }

    fn sah(triangles: &[Triangle], axis: &Axis, position: f32) -> f32 {
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

    fn best_split(triangles: &[Triangle]) -> Split {
        let mut best_split = Split::new(Axis::X, 0., f32::INFINITY);

        for axis in Axis::iter() {
            let mut bound = Interval::empty();
            for triangle in triangles {
                bound.min = bound.min.min(triangle.centroid[&axis]);
                bound.max = bound.max.max(triangle.centroid[&axis]);
            }
            if bound.min == bound.max {
                continue;
            }

            let num_intervals = 4;
            let scale = bound.size() / num_intervals as f32;
            for i in 1..num_intervals {
                let candidate_position = bound.min + i as f32 * scale;
                let cost = Self::sah(triangles, &axis, candidate_position);
                if cost <= best_split.cost {
                    best_split = Split::new(axis, candidate_position, cost);
                }
            }
        }

        return best_split;
    }

    fn subdivide(&mut self, node_index: usize) {
        let node = &self.nodes[node_index];

        let best_split = Self::best_split(&self.triangles[node.triangle_range()]);
        let parent_cost = node.aabb.area() * node.triangle_count as f32;
        if best_split.cost >= parent_cost {
            return;
        }

        let split_index = partition(&mut self.triangles[node.triangle_range()], |triangle| {
            triangle.centroid[&best_split.axis] <= best_split.position
        });
        if split_index == 0 || split_index == node.triangle_count {
            return;
        }

        let left_node = BvhNode::new(self, node.left_first, split_index);
        let right_node = BvhNode::new(
            self,
            node.left_first + split_index,
            node.triangle_count - split_index,
        );

        let left_index = self.nodes.len();
        self.nodes.push(left_node);
        self.nodes.push(right_node);
        self.subdivide(left_index);
        self.subdivide(left_index + 1);

        self.nodes[node_index].left_first = left_index;
        self.nodes[node_index].triangle_count = 0;

        return;
    }

    pub fn new(triangles: Vec<Triangle<'a>>) -> Self {
        let nodes = Vec::with_capacity(triangles.len() * 2 - 1);

        let mut bvh = Self { triangles, nodes };
        let root_node = BvhNode::new(&bvh, 0, bvh.triangles.len());

        bvh.nodes.push(root_node);
        bvh.subdivide(0);
        bvh.nodes.shrink_to_fit();

        return bvh;
    }

    fn intersect(&self, ray: &Ray, t_interval: &mut Interval, node_index: usize) -> Option<Hit> {
        let node = &self.nodes[node_index];
        if !node.aabb.hit(ray, t_interval) {
            return None;
        }

        if node.is_leaf() {
            return self.triangles[node.triangle_range()].hit(ray, t_interval);
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
    aabb: Aabb,
    left_first: usize,
    triangle_count: usize,
}
impl BvhNode {
    fn new(bvh: &Bvh, left_first: usize, triangle_count: usize) -> Self {
        return Self {
            aabb: bvh.triangles[left_first..left_first + triangle_count].aabb(),
            left_first,
            triangle_count,
        };
    }

    fn triangle_range(&self) -> Range<usize> {
        return self.left_first..self.left_first + self.triangle_count;
    }

    /// Returns whether the node is a leaf, i.e. whether it contains triangles
    fn is_leaf(&self) -> bool {
        return self.triangle_count > 0;
    }
}

impl Hittable for Bvh<'_> {
    fn aabb(&self) -> Aabb {
        return self.nodes[0].aabb;
    }

    fn hit(&self, ray: &Ray, t_interval: &mut Interval) -> Option<Hit> {
        return self.intersect(ray, t_interval, 0);
    }
}
