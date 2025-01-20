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
    fuzz: f32,
}
impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        return Self {
            albedo,
            fuzz: fuzz.clamp(0., 1.),
        };
    }
}
impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let mut reflected = ray_in.direction.reflect(&hit.normal);
        reflected = reflected.normalize() + (self.fuzz * Vec3::random_unit_vector());

        // Check whether the reflection has been fuzzed below the surface
        // If it has, have the surface absorb the ray
        if reflected.dot(&hit.normal) <= 0. {
            return None;
        }

        return Some((Ray::new(hit.point, reflected), self.albedo));
    }
}

#[derive(Debug)]
pub struct Dielectric {
    refraction_index: f32,
}
impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        return Self { refraction_index };
    }
}
impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let refractive_index_ratio = match hit.front_face {
            true => 1. / self.refraction_index,
            false => self.refraction_index,
        };

        let refracted = ray_in
            .direction
            .normalize()
            .refract(&hit.normal, refractive_index_ratio);

        return Some((Ray::new(hit.point, refracted), Color::new(1., 1., 1.)));
    }
}
