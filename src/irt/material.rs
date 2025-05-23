use rand::random;

use crate::irt::{Color, Hit, Point, Ray, Texture, UnitVec3};
use std::fmt::Debug;

pub trait Material: Debug + Sync {
    fn scatter(&self, _ray_in: &Ray, _hit: &Hit) -> Option<(Ray, Color)> {
        return None;
    }
    fn emitted(&self, _u: f32, _v: f32, _point: Point) -> Color {
        return Color::black();
    }
}

#[derive(Debug)]
pub struct Lambertian {
    texture: Box<dyn Texture>,
}
impl Lambertian {
    pub fn new(texture: Box<dyn Texture>) -> Self {
        return Self { texture };
    }
}
impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit.normal.as_vec3() + UnitVec3::random().as_vec3();
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal.as_vec3();
        }
        let scattered_ray = Ray::new(hit.point, scatter_direction);

        return Some((scattered_ray, self.texture.value(hit.u, hit.v, hit.point)));
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
        let mut reflected = ray_in.direction.reflect(hit.normal.as_vec3());
        reflected = reflected.normalize().as_vec3() + (self.fuzz * UnitVec3::random().as_vec3());

        // Check whether the reflection has been fuzzed below the surface
        // If it has, have the surface absorb the ray
        if reflected.dot(hit.normal.as_vec3()) <= 0. {
            return None;
        }

        return Some((Ray::new(hit.point, reflected), self.albedo));
    }
}

#[derive(Debug)]
pub struct Dielectric {
    /// The refractive index of the material in a vacuum.
    /// ALternatively, the refractive index in the material's enclosing media.
    refraction_index: f32,
}
impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        return Self { refraction_index };
    }

    /// Compute reflectance with Schlick's approximation
    fn reflectance(&self, cos_theta: f32) -> f32 {
        let r_0 = ((1. - self.refraction_index) / (1. + self.refraction_index)).powi(2);
        return r_0 + (1. - r_0) * (1. - cos_theta).powi(5);
    }
}
impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let refractive_index_ratio = match hit.front_face {
            true => 1. / self.refraction_index,
            false => self.refraction_index,
        };

        let unit_in_direction = ray_in.direction.normalize().as_vec3();
        let cos_theta = -unit_in_direction.dot(hit.normal.as_vec3());
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();

        let cannot_refract = refractive_index_ratio * sin_theta > 1.;
        let out_direction = match cannot_refract || self.reflectance(cos_theta) > random() {
            true => unit_in_direction.reflect(hit.normal.as_vec3()),
            false => unit_in_direction.refract(hit.normal.as_vec3(), refractive_index_ratio),
        };

        return Some((Ray::new(hit.point, out_direction), Color::white()));
    }
}

#[derive(Debug)]
pub struct DiffuseLight {
    texture: Box<dyn Texture>,
}
impl DiffuseLight {
    pub fn new(texture: Box<dyn Texture>) -> Self {
        return Self { texture };
    }
}
impl Material for DiffuseLight {
    fn emitted(&self, u: f32, v: f32, point: Point) -> Color {
        return self.texture.value(u, v, point);
    }
}
