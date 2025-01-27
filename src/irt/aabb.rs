use crate::{Point, Ray, Vec3};

/// Struct for an axis-aligned bounding box
#[derive(Debug)]
pub struct Aabb {
    pub min: Point,
    pub max: Point,
}
impl Aabb {
    pub fn new(min: Point, max: Point) -> Self {
        return Self { min, max };
    }

    pub fn empty() -> Self {
        return Self::new(
            Point::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            Point::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
        );
    }

    /// Returns the extent of the AABB,
    /// i.e. its length in the X, Y, and Z directions.
    pub fn extent(&self) -> Vec3 {
        return self.max - self.min;
    }

    pub fn expand(&self, other: &Self) -> Self {
        return Self::new(self.min.min(&other.min), self.max.max(&other.max));
    }

    pub fn expand_to_point(&mut self, point: &Point) {
        self.min = self.min.min(point);
        self.max = self.max.max(point);
    }

    pub fn hit(&self, ray: &Ray) -> bool {
        let inverse_ray_direction = 1. / ray.direction;
        let t_0 = (self.min - ray.origin) * inverse_ray_direction;
        let t_1 = (self.max - ray.origin) * inverse_ray_direction;

        let t_min = t_0.min(&t_1);
        let t_max = t_0.max(&t_1);

        return t_min.max_component() <= t_max.min_component();
    }

    pub fn area(&self) -> f32 {
        let extent = self.extent();
        return 2. * (extent.x * extent.y + extent.y * extent.z + extent.z * extent.x);
    }
}
