use crate::{ray::Ray, hittable::HitRecord, vec3::{Color, Vec3}};

use rand::prelude::*;

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Default for Lambertian {
    fn default() -> Self {
        Self::new(Color::default())
    }
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal
        }

        let new_scattered = Ray::new(rec.p, scatter_direction);

        scattered.origin = new_scattered.origin;
        scattered.direction = new_scattered.direction;

        *attenuation = self.albedo;

        true
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz
        }
    }
}

impl Default for Metal {
    fn default() -> Self {
        Self::new(Color::default(), 0.0)
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
       let reflected = r.direction.unit_vector().reflect(rec.normal);

        *scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());

        *attenuation = self.albedo;

        scattered.direction.dot(rec.normal) > 0.0
    }    
}


pub struct Dielectric {
    ir: f64
}

impl Default for Dielectric {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self {
            ir
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
        r0 = r0 * r0;

        r0 + (1.0-r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face  { 1.0 / self.ir } else { self.ir };

        let unit_direction =  r.direction.unit_vector();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let mut rng = thread_rng();

        let direction = if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>(){
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, refraction_ratio)
        };

        *scattered = Ray::new(rec.p, direction);

        true
    }
}
