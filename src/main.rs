use std::sync::Arc;

use rayon::prelude::*;

mod vec3;
use vec3::*;

mod camera;

mod obj;
use obj::*;

mod hit;
use hit::*;

mod bvh;

mod material;
use material::*;

mod texture;
use texture::*;

mod util;
use util::*;

mod perlin;

mod pdf;
use pdf::*;

mod transf;

mod scene;
use scene::*;

fn color(r: Ray, world: &Vec<Arc<dyn Hittable>>, depth: u32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if let Some(hit) = world.hit(r, 0.001, std::f32::MAX) {
        let emitted = hit.material.emitted(&r, &hit, hit.u, hit.v, &hit.p);
        if let Some(s_rec) = hit.material.scatter(r, &hit) {
            let light_shape = Arc::new(XZRect::new(
                213.0,
                343.0,
                227.0,
                332.0,
                554.0,
                Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(
                    Vec3::new(1.0, 1.0, 1.0),
                )))),
            ));
            let p = HittablePdf::new(light_shape, hit.p);
            let scattered = Ray::new(hit.p, p.generate(), r.time());
            let pdf_val = p.value(&scattered.direction());

            return emitted
                + s_rec.albedo
                    * hit.material.scattering_pdf(&r, &hit, &scattered)
                    * color(scattered, world, depth - 1)
                    / pdf_val;
        } else {
            return emitted;
        }
    }
    Vec3::new(0.0, 0.0, 0.0)
}

fn main() {
    let (nx, ny, ns) = (500, 500, 10);
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let (cam, world) = cornell_mc(ny as f32 / nx as f32);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col: Vec3 = (0..ns)
                .into_par_iter()
                .map(|_| {
                    let u: f32 = (i as f32 + rand_float()) / nx as f32;
                    let v: f32 = (j as f32 + rand_float()) / ny as f32;
                    let r = cam.get_ray(u, v);
                    color(r, &world, 50)
                })
                .sum();
            col /= ns as f32;
            col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());

            let ir: u32 = (255.99 * col[0]) as u32;
            let ig: u32 = (255.99 * col[1]) as u32;
            let ib: u32 = (255.99 * col[2]) as u32;
            println!("{} {} {}", ir, ig, ib)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::*;

    #[test]
    fn mc() {
        let n = 1000000;
        let mut sum = 0.0;
        for i in 0..n {
            let r1 = rand_float();
            let r2 = rand_float();
            let x = (2.0 * std::f32::consts::PI * r1).cos() * 2.0 * (r2 * (1.0 - r2)).sqrt();
            let y = (2.0 * std::f32::consts::PI * r1).sin() * 2.0 * (r2 * (1.0 - r2)).sqrt();
            let z = 1.0 - r2;
            sum += z * z * z / (1.0 / (2.0 * std::f32::consts::PI));
        }
        println!("Pi/2 = {}", std::f32::consts::PI / 2.0 as f32);
        println!("Estimate = {}", sum / n as f32);
    }
}
