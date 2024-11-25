use crate::hittable::HitRecord;
// Import necessary modules
use crate::ray::Ray;
use crate::utils::*;
use rand::Rng;

use glam::DVec3;

// Material enum defines different material types

#[derive(Default, Copy, Clone)]
pub enum Material {
    #[default]
    Default,
    Lambertian {
        albedo: DVec3,
    },
    Metal {
        albedo: DVec3,
        fuzz: f64,
    },
    Dielectric {
        refraction_index: f64,
    },
}

impl Material {
    pub fn scatter(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        attenuation: &mut DVec3,
        scattered: &mut Ray,
    ) -> Option<bool> {
        use Material::*;

        match self {
            Default {} => Some(true),
            Lambertian { albedo } => {
                let mut scatter_direction = rec.normal + random_dvec3_unit();

                if near_zero(&scatter_direction) {
                    scatter_direction = rec.normal;
                }

                *scattered = Ray::new(rec.p, scatter_direction);
                *attenuation = *albedo;
                Some(true)
            }
            Metal { albedo, fuzz } => {
                let fuzz = if *fuzz > 1.0 { 1.0 } else { *fuzz };
                let reflected = reflect(r_in.direction, rec.normal).unwrap();
                let reflected = reflected.normalize() + (fuzz * random_dvec3_unit());
                *scattered = Ray::new(rec.p, reflected);
                *attenuation = *albedo;
                Some(scattered.direction.dot(rec.normal) > 0.0)
            }
            Dielectric { refraction_index } => {
                // White, no tinting
                *attenuation = DVec3::new(1.0, 1.0, 1.0);

                let unit_direction = r_in.direction.normalize();

                let cos_theta = rec.normal.dot(-1.0 * unit_direction).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let ri = if rec.front_face {
                    1.0 / *refraction_index
                } else {
                    *refraction_index
                };

                let cannot_refract = ri * sin_theta > 1.0;
                let mut rng = rand::thread_rng();
                let direction =
                    if cannot_refract || Self::reflectance(cos_theta, ri) > rng.gen::<f64>() {
                        reflect(unit_direction, rec.normal).unwrap()
                    } else {
                        refract(&rec.normal, &unit_direction, ri).unwrap()
                    };

                *scattered = Ray::new(rec.p, direction);

                Some(true)
            }
        }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}
