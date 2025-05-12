use std::ops::{Add, AddAssign};

use crate::irt::{Axis, Interval, Point, Ray, Vec3};

/// Struct for an axis-aligned bounding box
#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: Point,
    pub max: Point,
}
impl Aabb {
    const MINIMUM_SIZE: f32 = 0.0001;
    fn pad_to_minimums(&mut self) {
        for axis in Axis::iter() {
            let delta = self.max[&axis] - self.min[&axis];
            if delta < Aabb::MINIMUM_SIZE {
                self.min[&axis] -= Aabb::MINIMUM_SIZE / 2.;
                self.max[&axis] += Aabb::MINIMUM_SIZE / 2.;
            }
        }
    }

    pub fn new(min: Point, max: Point) -> Self {
        let mut aabb = Self { min, max };
        aabb.pad_to_minimums();
        return aabb;
    }

    pub fn empty() -> Self {
        return Self::new(
            Point::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            Point::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
        );
    }

    /// Returns an array of the 8 corners that form the AABB.
    pub fn corners(&self) -> [Point; 8] {
        return [
            self.min,
            Point::new(self.min.x, self.min.y, self.max.z),
            Point::new(self.min.x, self.max.y, self.min.z),
            Point::new(self.max.x, self.min.y, self.min.z),
            Point::new(self.min.x, self.max.y, self.max.z),
            Point::new(self.max.x, self.min.y, self.max.z),
            Point::new(self.max.x, self.max.y, self.min.z),
            self.max,
        ];
    }

    /// Returns the extent of the AABB,
    /// i.e. its length in the X, Y, and Z directions.
    pub fn extent(&self) -> Vec3 {
        return self.max - self.min;
    }

    #[allow(dead_code)]
    pub fn expand_to_point(&mut self, point: Point) {
        self.min = self.min.min(point);
        self.max = self.max.max(point);
    }

    pub fn hit(&self, ray: &Ray, t_interval: &Interval) -> bool {
        let inverse_ray_direction = 1. / ray.direction;
        let t_0 = (self.min - ray.origin) * inverse_ray_direction;
        let t_1 = (self.max - ray.origin) * inverse_ray_direction;

        let t_min = t_0.min(t_1).max_component();
        let t_max = t_0.max(t_1).min_component();

        return t_interval.min.max(t_min) <= t_interval.max.min(t_max);
    }

    pub fn area(&self) -> f32 {
        let extent = self.extent();
        return 2. * (extent.x * extent.y + extent.y * extent.z + extent.z * extent.x);
    }
}

impl Add for Aabb {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        return Self::new(self.min.min(other.min), self.max.max(other.max));
    }
}
impl AddAssign for Aabb {
    fn add_assign(&mut self, other: Self) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
    }
}
