use crate::hittable::HitRecord;
// Import necessary modules
use crate::ray::Ray;
use crate::utils::{near_zero, random_dvec3_unit};
use rand::Rng;

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
    pub fn scatter(&self, r_in: Ray, rec: &HitRecord, debug: bool) -> Option<(DVec3, Ray, bool)> {
        use Material::*;

        match self {
            Default {} => Some((
                DVec3::ZERO,
                Ray::with_direction(DVec3::ONE, DVec3::ONE),
                true,
            )),
            Lambertian { albedo } => {
                let mut scatter_direction = rec.normal + random_dvec3_unit();

                if near_zero(&scatter_direction) {
                    scatter_direction = rec.normal;
                }

                let scattered = Ray::with_time(rec.p, scatter_direction, r_in.time);
                let attenuation = *albedo;
                Some((attenuation, scattered, true))
            }
            Metal { albedo, fuzz } => {
                let fuzz = if *fuzz > 1.0 { 1.0 } else { *fuzz };
                let reflected = Self::reflect(r_in.direction, rec.normal).unwrap();
                let reflected = reflected + (fuzz * random_dvec3_unit());

                let scattered = Ray::with_time(rec.p, reflected, r_in.time);
                let attenuation = *albedo;

                // Returns false if a fuzzed ray ends up bouncing inside the surface
                Some((
                    attenuation,
                    scattered,
                    scattered.direction.dot(rec.normal) > 0.0,
                ))
            }
            Dielectric { refraction_index } => {
                let mut rng = rand::thread_rng();

                // White, no tinting
                let attenuation = DVec3::ONE;

                let ri = if rec.front_face {
                    refraction_index.recip()
                } else {
                    *refraction_index
                };

                let unit_direction = r_in.direction.normalize();

                let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let cannot_refract = ri * sin_theta > 1.0;

                let direction =
                    if cannot_refract || Self::reflectance(cos_theta, ri) > rng.gen::<f64>() {
                        Self::reflect(unit_direction, rec.normal).unwrap()
                    } else {
                        Self::refract(unit_direction, rec.normal, ri).unwrap()
                    };

                let scattered = Ray::with_time(rec.p, direction, r_in.time);
                if debug {
                    println!("material::scatter::scattered: {:?}", scattered);
                    println!("material::scatter::direction: {:?}", direction);
                }
                Some((attenuation, scattered, true))
            }
        }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }

    pub fn reflect(v: DVec3, n: DVec3) -> Option<DVec3> {
        Some(v - 2.0 * v.dot(n) * n)
    }

    pub fn refract(v: DVec3, n: DVec3, ni_over_nt: f64) -> Option<DVec3> {
        let cos_theta = (-1.0 * v).dot(n).min(1.0);
        let r_out_perp = ni_over_nt * (v + cos_theta * n);
        let r_out_parallel = (-1.0 * (1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;
        Some(r_out_perp + r_out_parallel)
    }
}

#[cfg(test)]
mod tests {
    use crate::material;

    use super::*;

    #[test]
    fn refract_test() {
        let r_in = DVec3::new(1.0, -1.0, 0.0).normalize();
        let normal = DVec3::new(0.0, 1.0, 0.0);

        let refract = Material::refract(r_in, normal, 1.0).unwrap();

        assert!((r_in - refract).length().abs() < f64::EPSILON);

        let r_in = DVec3::new(1.0, -1.0, 0.0).normalize();
        let normal = DVec3::new(0.0, 1.0, 0.0);

        let refract = Material::refract(r_in, normal, 1.0 / 1.5).unwrap();
        let expected = DVec3::new((2.0_f64.sqrt()) / 3.0, -(7.0_f64.sqrt()) / 3.0, 0.0).normalize();

        assert!((expected - refract).length().abs() < f64::EPSILON);
    }

    #[test]
    fn scatter_metallic_no_fuzz_test() {
        let rec = HitRecord {
            p: DVec3::ZERO,
            normal: DVec3::Y,
            t: 1.0,
            front_face: true,
            mat: Material::Metal {
                albedo: (DVec3::new(0.8, 0.1, 0.0)),
                fuzz: 0.0,
            },
        };

        let r = Ray::with_time(
            DVec3::new(-1.0, 1.0, 0.0),
            DVec3::new(1.0, -1.0, 0.0).normalize(),
            0.0,
        );

        let (_attenuation, scattered, _keeps_bouncing) = rec.mat.scatter(r, &rec, false).unwrap();

        let reflected = DVec3::new(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);

        assert!((scattered.direction - reflected).length().abs() < f64::EPSILON);
    }

    #[test]
    fn scatter_dielectric_ref_1_5_entry_test() {
        let rec = HitRecord {
            p: DVec3::ZERO,
            normal: DVec3::Y,
            t: 1.0,
            front_face: true,
            mat: Material::Dielectric {
                refraction_index: (1.5),
            },
        };

        let r = Ray::with_time(
            DVec3::new(-1.0, 1.0, 0.0),
            DVec3::new(1.0, -1.0, 0.0).normalize(),
            0.0,
        );

        let (_attenuation, scattered, _keeps_bouncing) = rec.mat.scatter(r, &rec, false).unwrap();

        let _reflected = DVec3::new(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);

        let expected = DVec3::new((2.0_f64.sqrt()) / 3.0, -(7.0_f64.sqrt()) / 3.0, 0.0).normalize();
        // assert_eq!(scattered.direction, expected);
        assert!((scattered.direction - expected).length().abs() < f64::EPSILON);
    }

    #[test]
    fn scatter_dielectric_ref_1_5_exit_test() {
        let rec = HitRecord {
            p: DVec3::ZERO,
            normal: DVec3::Y,
            t: 1.0,
            front_face: false,
            mat: Material::Dielectric {
                refraction_index: (1.5),
            },
        };

        let r = Ray::with_time(
            DVec3::new(-1.0, 1.0, 0.0),
            DVec3::new((2.0_f64.sqrt()) / 3.0, -(7.0_f64.sqrt()) / 3.0, 0.0).normalize(),
            0.0,
        );

        let (_attenuation, scattered, _keeps_bouncing) = rec.mat.scatter(r, &rec, false).unwrap();

        let _reflected = DVec3::new(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);

        let expected = DVec3::new(1.0, -1.0, 0.0).normalize();

        assert!((scattered.direction - expected).length().abs() < 0.1);
    }

    #[test]
    fn scatter_dielectric_ref_1_0_dead_on_test() {
        let rec = HitRecord {
            p: DVec3::ZERO,
            normal: DVec3::Y,
            t: 1.0,
            front_face: false,
            mat: Material::Dielectric {
                refraction_index: (1.5),
            },
        };

        let ray = Ray::with_time(
            DVec3::new(0.0, 1.0, 0.0),
            DVec3::new(0.0, -1.0, 0.0).normalize(),
            0.0,
        );

        let (_attenuation, scattered, _keeps_bouncing) = rec.mat.scatter(ray, &rec, false).unwrap();

        let expected = DVec3::new(0.0, -1.0, 0.0).normalize();

        assert!((scattered.direction - expected).length().abs() < f64::EPSILON);
    }
}
