use rand::seq::SliceRandom;

use crate::util::*;
use crate::vec3::{dot, Vec3};

pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<u32>,
    perm_y: Vec<u32>,
    perm_z: Vec<u32>,
}

impl Perlin {
    pub fn new() -> Self {
        Self {
            ranvec: Perlin::generate(),
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

        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let index = (self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]) as usize;
                    c[di][dj][dk] = self.ranvec[index];
                }
            }
        }
        Perlin::interp(c, u, v, w)
    }

    fn generate() -> Vec<Vec3> {
        let mut v: Vec<Vec3> = Vec::new();
        for _ in 0..256 {
            let x_rand = 2.0 * rand_float() - 1.0;
            let y_rand = 2.0 * rand_float() - 1.0;
            let z_rand = 2.0 * rand_float() - 1.0;
            v.push(Vec3::new(x_rand, y_rand, z_rand).unit());
        }
        v
    }

    fn generate_perm() -> Vec<u32> {
        let mut p: Vec<u32> = (0..256).collect();
        p.shuffle(&mut rand::thread_rng());
        p
    }

    fn interp(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                    accum += (i as f32 * uu + (1.0 - i as f32) * (1.0 - uu))
                        * (j as f32 * vv + (1.0 - j as f32) * (1.0 - vv))
                        * (k as f32 * ww + (1.0 - k as f32) * (1.0 - ww))
                        * dot(c[i][j][k], weight_v);
                }
            }
        }
        accum
    }

    pub fn turb(&self, p: &Vec3, depth: u32) -> f32 {
        let mut accum = 0.0;
        let mut weight = 1.0;
        let mut temp_p = Vec3::new(p.x(), p.y(), p.z());
        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }
}
