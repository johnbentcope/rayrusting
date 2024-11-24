use glam::DVec3;
use rand::Rng;

// pub struct Utils {}

// impl Utils{
pub fn _random_dvec3() -> DVec3 {
    let mut rng = rand::thread_rng();
    DVec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()).normalize()
}

pub fn random_dvec3_range(min: f64, max: f64) -> DVec3 {
    let mut rng = rand::thread_rng();
    DVec3::new(
        rng.gen_range(min..=max),
        rng.gen_range(min..=max),
        rng.gen_range(min..=max),
    )
    .normalize()
}

pub fn random_dvec3_unit() -> DVec3 {
    loop {
        let p = random_dvec3_range(-1.0, 1.0);
        let len_sq = p.length_squared();
        if 1e-160 < len_sq && len_sq <= 1.0 {
            return p / len_sq.sqrt();
        }
    }
}

pub fn random_on_hemisphere(normal: DVec3) -> DVec3 {
    let on_unit_sphere = random_dvec3_unit();
    if on_unit_sphere.dot(normal) > 0.0 {
        // In the same hemisphere as the normal
        return on_unit_sphere;
    } else {
        return -on_unit_sphere;
    }
}

pub fn near_zero(test: &DVec3) -> bool {
    test.length() < 0.00001
}

pub fn reflect(v: DVec3, n: DVec3) -> DVec3 {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(uv: DVec3, n: DVec3, etai_over_etat: f64) -> DVec3 {
    let cos_theta = ((-1.0 * uv).dot(n)).min(1.0);
    let r_out_perp = etai_over_etat * (uv + (cos_theta * n));
    let r_out_parallel = -(1.0 * ((1.0 - r_out_perp.length_squared()).abs()).sqrt()) * n;
    r_out_perp + r_out_parallel
}

// pub fn random_in_unit_sphere() -> DVec3 {
//     let mut rng = rand::thread_rng();
//     let mut v = DVec3::new(0.0, 0.0, 0.0);
//     while v.length() >= 1.0 {
//         v = DVec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 2.0
//             - DVec3::new(1.0, 1.0, 1.0);
//     }
//     v
// }
