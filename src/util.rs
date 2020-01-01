use crate::vec3::Vec3;
use rand::Rng;

pub fn rand_float() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
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
        let p = 2.0 * Vec3::new(rand_float(), rand_float(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if p.mag() < 1.0 {
            return p;
        }
    }
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub fn trilinear_interp(c: [[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                accum += (i as f32 * u as f32 + (1.0 - i as f32) * (1.0 - u as f32))
                    * (j as f32 * v as f32 + (1.0 - j as f32) * (1.0 - v as f32))
                    * (k as f32 * w as f32 + (1.0 - k as f32) * (1.0 - w as f32))
                    * c[i as usize][j as usize][k as usize];
            }
        }
    }
    accum
}
