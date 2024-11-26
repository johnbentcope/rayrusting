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

pub fn random_in_unit_disc() -> DVec3 {
    let mut rng = rand::thread_rng();
    DVec3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0).normalize()
}

pub fn near_zero(test: &DVec3) -> bool {
    test.length() < 0.00001
}
