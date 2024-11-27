use crate::hittable::HitRecord;
// Import necessary modules
use crate::ray::Ray;
use crate::utils::*;
// use rand::Rng;

use glam::DVec3;

// Material enum defines different material types

#[derive(Debug, Default, Copy, Clone)]
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
    pub fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(DVec3, Ray, bool)> {
        use Material::*;

        match self {
            Default {} => Some((
                DVec3::new(0.0, 0.0, 1.0),
                Ray::new(DVec3::new(0.0, 0.0, 1.0), DVec3::new(0.0, 0.0, 1.0)),
                true,
            )),
            Lambertian { albedo } => {
                let mut scatter_direction = rec.normal + random_dvec3_unit();

                if near_zero(&scatter_direction) {
                    scatter_direction = rec.normal;
                }

                let scattered = Ray::new(rec.p, scatter_direction);
                let attenuation = *albedo;
                Some((attenuation, scattered, true))
            }
            Metal { albedo, fuzz } => {
                let fuzz = if *fuzz > 1.0 { 1.0 } else { *fuzz };
                let reflected = Self::reflect(r_in.direction, rec.normal).unwrap();
                let reflected = reflected.normalize() + (fuzz * random_dvec3_unit());

                let scattered = Ray::new(rec.p, reflected);
                let attenuation = *albedo;
                Some((
                    attenuation,
                    scattered,
                    scattered.direction.dot(rec.normal) > 0.0,
                ))
            }
            Dielectric { refraction_index } => {
                // White, no tinting
                let attenuation = DVec3::new(0.9, 0.9, 0.9);

                let ri = if rec.front_face {
                    1.0 / (*refraction_index)
                } else {
                    *refraction_index
                };

                let unit_direction = r_in.direction.normalize();
                let refracted = Self::refract(&unit_direction, &rec.normal, ri).unwrap();

                let scattered = Ray::new(rec.p, refracted);

                // println!("r_in.direction: {:?}\nrefracted: {:?}",r_in.direction, refracted);
                // println!(" scattered: {:?}\n rec.p: {:?}",scattered, rec.p);

                Some((attenuation, scattered, true))
            }
        }
    }

    // fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    //     // Use Schlick's approximation for reflectance.
    //     let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    //     let r0 = r0 * r0;
    //     r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    // }

    pub fn reflect(v: DVec3, n: DVec3) -> Option<DVec3> {
        Some(v - 2.0 * v.dot(n) * n)
    }

    pub fn refract(v: &DVec3, n: &DVec3, ni_over_nt: f64) -> Option<DVec3> {
        let uv = v.normalize();
        let cos_theta = ((-1.0 * (uv)).dot(*n)).min(1.0);
        let r_out_perp = ni_over_nt * (uv + (cos_theta * (*n)));
        let r_out_parallel = (-1.0 * (1.0 - r_out_perp.length_squared()).abs().sqrt()) * (*n);
        Some(r_out_perp + r_out_parallel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refraction_unit_test() {
        let r_in = DVec3::new(1.0, -1.0, 0.0).normalize();
        let normal = DVec3::new(0.0, 1.0, 0.0);

        let refract = Material::refract(&r_in, &normal, 1.0).unwrap();

        assert!((r_in - refract).length().abs() < f64::EPSILON);

        let r_in = DVec3::new(1.0, -1.0, 0.0).normalize();
        let normal = DVec3::new(0.0, 1.0, 0.0);

        let refract = Material::refract(&r_in, &normal, 1.0 / 1.5).unwrap();
        let expected = DVec3::new((2.0_f64.sqrt()) / 3.0, -(7.0_f64.sqrt()) / 3.0, 0.0).normalize();

        assert!((expected - refract).length().abs() < f64::EPSILON);
    }
}
