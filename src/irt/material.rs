use rand::random;

use crate::{Color, Hit, Ray, Texture, Vec3};
use std::fmt::Debug;

pub trait Material: Debug + Sync {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<(Ray, Color)>;
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
        let mut scatter_direction = hit.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
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

        let unit_in_direction = ray_in.direction.normalize();
        let cos_theta = -unit_in_direction.dot(&hit.normal);
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();

        let cannot_refract = refractive_index_ratio * sin_theta > 1.;
        let out_direction = match cannot_refract || self.reflectance(cos_theta) > random() {
            true => unit_in_direction.reflect(&hit.normal),
            false => unit_in_direction.refract(&hit.normal, refractive_index_ratio),
        };

        return Some((Ray::new(hit.point, out_direction), Color::white()));
    }
}
