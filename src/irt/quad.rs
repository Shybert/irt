use crate::irt::{Aabb, Hit, Hittable, Interval, Material, Point, Ray, UnitVec3, Vec3};

/// A quadrilateral (techinally a parallellogram).
/// Defined by:
///
/// 1. A [Point] `Q` defining the starting corner
/// 2. A [Vec3] `u` representing one side of the quad
/// 3. A [Vec3] `v` representing the other, non-parallell side of the quad.
#[derive(Debug)]
pub struct Quad<'a> {
    /// The starting corner
    q: Point,
    /// A side of the quad
    u: Vec3,
    /// A side of the quad
    v: Vec3,
    /// Pre-calculated used to determine the planar coordinates of an intersection
    w: Vec3,
    material: &'a dyn Material,
    bounds: Aabb,
    normal: UnitVec3,
    /// The `d` in the general equation of a plane, pre-calculated
    d: f32,
}
impl<'a> Quad<'a> {
    pub fn new(q: Point, u: Vec3, v: Vec3, material: &'a dyn Material) -> Self {
        let bounds_diagonal1 = Aabb::new(q, q + u + v);
        let bounds_diagonal2 = Aabb::new(q + u, q + v);
        let bounds = bounds_diagonal1 + bounds_diagonal2;

        let n = u.cross(v);
        let normal = n.normalize();
        let d = normal.as_vec3().dot(q.into());
        let w = n / n.dot(n);

        return Self {
            q,
            u,
            v,
            w,
            material,
            bounds,
            normal,
            d,
        };
    }

    /// Returns a 1x1x1 cube with corners in (0, 0, 0) and (1, 1, 1).
    pub fn cube(material: &'a dyn Material) -> Vec<Self> {
        let min = Point::new(0., 0., 0.);
        let max = Point::new(1., 1., 1.);

        let dx = Vec3::new(max.x - min.x, 0., 0.);
        let dy = Vec3::new(0., max.y - min.y, 0.);
        let dz = Vec3::new(0., 0., max.z - min.z);

        return vec![
            Quad::new(Point::new(min.x, min.y, max.z), dx, dy, material),
            Quad::new(Point::new(max.x, min.y, max.z), -dz, dy, material),
            Quad::new(Point::new(max.x, min.y, min.z), -dx, dy, material),
            Quad::new(Point::new(min.x, min.y, min.z), dz, dy, material),
            Quad::new(Point::new(min.x, max.y, max.z), dx, -dz, material),
            Quad::new(Point::new(min.x, min.y, min.z), dx, dz, material),
        ];
    }

    /// Given a hit point in the planar coordinates `a` and `b`,
    /// returns whether the point is inside the quadrilateral.
    fn is_interior(&self, a: f32, b: f32) -> bool {
        let unit_interval = Interval::new(0., 1.);
        return unit_interval.surrounds(a) && unit_interval.surrounds(b);
    }
}

impl Hittable for Quad<'_> {
    fn bounds(&self) -> Aabb {
        return self.bounds;
    }
    fn hit(&self, ray: &Ray, t_interval: &mut Interval) -> Option<Hit> {
        let denominator = self.normal.as_vec3().dot(ray.direction);

        // Return `None` if the ray is parallell to the plane.
        if denominator.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.as_vec3().dot(ray.origin.into())) / denominator;
        if !t_interval.surrounds(t) {
            return None;
        }

        let point = ray.at(t);

        let relative_p = point - self.q;
        let a = self.w.dot(relative_p.cross(self.v));
        let b = self.w.dot(self.u.cross(relative_p));
        if !self.is_interior(a, b) {
            return None;
        }

        return Some(Hit::new(ray, point, self.normal, t, self.material, a, b));
    }
}
