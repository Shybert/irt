use std::ops::Index;

use strum::IntoEnumIterator;

use crate::{Axis, Interval, Point, Ray};

/// Struct for an axis-aligned bounding box
#[derive(Debug)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}
impl Aabb {
    pub fn from_intervals(x: Interval, y: Interval, z: Interval) -> Self {
        return Self { x, y, z };
    }

    pub fn new(a: Point, b: Point) -> Self {
        let x = if a.x <= b.x {
            Interval::new(a.x, b.x)
        } else {
            Interval::new(b.x, a.x)
        };

        let y = if a.y <= b.y {
            Interval::new(a.y, b.y)
        } else {
            Interval::new(b.y, a.y)
        };

        let z = if a.z <= b.z {
            Interval::new(a.z, b.z)
        } else {
            Interval::new(b.z, a.z)
        };

        return Self::from_intervals(x, y, z);
    }

    pub fn empty() -> Self {
        return Self::from_intervals(Interval::empty(), Interval::empty(), Interval::empty());
    }

    pub fn expand(&self, other: &Self) -> Self {
        return Self::from_intervals(
            Interval::from_intervals(&self.x, &other.x),
            Interval::from_intervals(&self.y, &other.y),
            Interval::from_intervals(&self.z, &other.z),
        );
    }

    pub fn longest_axis(&self) -> Axis {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                return Axis::X;
            } else {
                return Axis::Z;
            };
        } else if self.y.size() > self.z.size() {
            return Axis::Y;
        } else {
            return Axis::Z;
        };
    }

    pub fn hit(&self, ray: &Ray, t_interval: &Interval) -> bool {
        let mut t_min = t_interval.min;
        let mut t_max = t_interval.max;

        for axis in Axis::iter() {
            let axis_interval = &self[&axis];
            let inv = 1. / ray.direction[&axis];

            let t_0 = (axis_interval.min - ray.origin[&axis]) * inv;
            let t_1 = (axis_interval.max - ray.origin[&axis]) * inv;

            t_min = t_0.min(t_1).max(t_min);
            t_max = t_0.max(t_1).min(t_max);
        }

        return t_min < t_max;
    }
}
impl Index<&Axis> for Aabb {
    type Output = Interval;

    fn index(&self, axis: &Axis) -> &Self::Output {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }
}
