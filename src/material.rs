use crate::hittable::HitRecord;
// Import necessary modules
use crate::ray::Ray;
use crate::utils::*;

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
                let reflected = reflect(r_in.direction, rec.normal);
                let reflected = reflected.normalize() + (fuzz * random_dvec3_unit());
                *scattered = Ray::new(rec.p, reflected);
                *attenuation = *albedo;
                Some(scattered.direction.dot(rec.normal) > 0.0)
            }
            Dielectric { refraction_index } => {
                *attenuation = DVec3::new(1.0, 1.0, 1.0);
                let ri = if rec.front_face {
                    1.0 / refraction_index
                } else {
                    *refraction_index
                };

                let unit_direction = r_in.direction.normalize();

                let cos_theta = (-1.0 * unit_direction).dot(rec.normal).min(1.0);
                let sin_theta = (1.0- cos_theta*cos_theta).sqrt();

                let cannot_refract = ri * sin_theta > 1.0;
                let direction = if cannot_refract {
                    reflect(unit_direction, rec.normal)
                } else {
                    refract(unit_direction, rec.normal, ri)
                };

                *scattered = Ray::new(rec.p, direction);

                Some(true)
            }
        }
    }
}
