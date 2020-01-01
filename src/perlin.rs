use rand::seq::SliceRandom;

use crate::util::*;
use crate::vec3::Vec3;

pub struct Perlin {
    ranfloat: Vec<f32>,
    perm_x: Vec<u32>,
    perm_y: Vec<u32>,
    perm_z: Vec<u32>,
}

impl Perlin {
    pub fn new() -> Self {
        Self {
            ranfloat: Perlin::generate(),
            perm_x: Perlin::generate_perm(),
            perm_y: Perlin::generate_perm(),
            perm_z: Perlin::generate_perm(),
        }
    }

    pub fn noise(&self, p: &Vec3) -> f32 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as usize;
        let j = p.y().floor() as usize;
        let k = p.z().floor() as usize;

        let mut c = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let index = (self.perm_x[(i+di) & 255] ^
                                 self.perm_y[(j+dj) & 255] ^
                                 self.perm_z[(k+dk) & 255]) as usize;
                    c[di][dj][dk] = self.ranfloat[index];
                }
            }
        }
        trilinear_interp(c, u, v, w)
    }

    fn generate() -> Vec<f32> {
        (0..256).map(|_| rand_float()).collect()
    }

    fn generate_perm() -> Vec<u32> {
        let mut p: Vec<u32> = (0..256).collect();
        p.shuffle(&mut rand::thread_rng());
        p
    }
}
