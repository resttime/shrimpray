use rand::Rng;
use crate::vec3::Vec3;

pub fn rand_float() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen::<f32>()
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p =
            2.0 * Vec3::new(rand_float(), rand_float(), rand_float()) - Vec3::new(1.0, 1.0, 1.0);
        if p.mag() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p =
            2.0 * Vec3::new(rand_float(), rand_float(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if p.mag() < 1.0 {
            return p;
        }
    }
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
    r0 = r0 * r0;

    r0 + (1.0-r0) * (1.0-cosine).powi(5)
}
