use crate::{Color, Hit, Ray, Vec3};
use std::fmt::Debug;

pub trait Material: Debug {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Color)>;
}

#[derive(Debug)]
pub struct Lambertian {
    albedo: Color,
}
impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        return Self { albedo };
    }
}
impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }
        let scattered_ray = Ray::new(hit.point, scatter_direction);

        return Some((scattered_ray, self.albedo));
    }
}

#[derive(Debug)]
pub struct Metal {
    albedo: Color,
}
impl Metal {
    pub fn new(albedo: Color) -> Self {
        return Self { albedo };
    }
}
impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let reflected = ray_in.direction.reflect(&hit.normal);
        return Some((Ray::new(hit.point, reflected), self.albedo));
    }
}
