use rand::seq::SliceRandom;

use crate::util::*;
use crate::vec3::Vec3;

struct Perlin {
    ranfloat: Vec<f32>,
    perm_x: Vec<u32>,
    perm_y: Vec<u32>,
    perm_z: Vec<u32>,
}

impl Perlin {
    fn new() -> Self {
        Self {
            ranfloat: (0..256).map(|_| rand_float()).collect(),
            perm_x: Perlin::generate_perm(),
            perm_y: Perlin::generate_perm(),
            perm_z: Perlin::generate_perm(),
        }
    }

    fn noise(&self, p: &Vec3) -> f32 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as usize;
        let j = p.y().floor() as usize;
        let k = p.z().floor() as usize;

        let index = (self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize;
        self.ranfloat[index]
    }

    fn generate_perm() -> Vec<u32> {
        let mut p: Vec<u32> = (0..256).collect();
        p.shuffle(&mut rand::thread_rng());
        p
    }
}
