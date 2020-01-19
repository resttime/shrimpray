use std::sync::Arc;

use rayon::prelude::*;

mod vec3;
use vec3::{Ray, Vec3};

mod camera;
use camera::Camera;

mod obj;

mod hit;
use hit::*;

mod bvh;

mod material;

mod texture;

mod util;
use util::*;

mod perlin;

mod transf;

mod scene;
use scene::*;

fn color(r: Ray, world: &Vec<Arc<dyn Hittable>>, depth: u32) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.001, std::f32::MAX) {
        let emitted = hit.material.emitted(hit.u, hit.v, &hit.p);
        if depth < 50 {
            if let Some((scattered, attenuation)) = hit.material.scatter(r, &hit) {
                return emitted + attenuation * color(scattered, world, depth + 1);
            } else {
                return emitted;
            }
        } else {
            return emitted;
        }
    }
    Vec3::new(0.0, 0.0, 0.0)
}

fn main() {
    let (nx, ny, ns) = (500, 500, 10000);
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let lookfrom = Vec3::new(478.0, 278.0, -600.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        vfov,
        nx as f32 / ny as f32,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let world = final_scene(); //BvhNode::new(&mut cornell_box(), 0.0, 1.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::default();
            for _ in 0..ns {
                let u: f32 = (i as f32 + rand_float()) / nx as f32;
                let v: f32 = (j as f32 + rand_float()) / ny as f32;
                let r = cam.get_ray(u, v);
                col += color(r, &world, 0);
            }
            col /= ns as f32;
            col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());

            let ir: u32 = (255.99 * col[0]) as u32;
            let ig: u32 = (255.99 * col[1]) as u32;
            let ib: u32 = (255.99 * col[2]) as u32;
            println!("{} {} {}", ir, ig, ib)
        }
    }
}
