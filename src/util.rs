use crate::vec3::Vec3;
use rand::Rng;

#[inline]
pub fn rand_float() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

#[inline]
pub fn rand_float_range(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}

#[inline]
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p =
            2.0 * Vec3::new(rand_float(), rand_float(), rand_float()) - Vec3::new(1.0, 1.0, 1.0);
        if p.mag() < 1.0 {
            return p;
        }
    }
}

#[inline]
pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(rand_float(), rand_float(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if p.mag() < 1.0 {
            return p;
        }
    }
}

#[inline]
pub fn random_unit_vector() -> Vec3 {
    let a = rand_float() * 2.0 * std::f32::consts::PI;
    let z = rand_float() * 2.0 - 1.0;
    let r = (1.0 - z*z).sqrt();
    return Vec3::new(r*a.cos(), r*a.sin(), z);
}

#[inline]
pub fn random_cosine_direction() -> Vec3 {
    let r1 = rand_float();
    let r2 = rand_float();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * std::f32::consts::PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3::new(x, y, z)
}

#[inline]
pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
