use rand::distributions::Standard;
use rand::prelude::*;

mod vec3;
use vec3::{dot, Ray, Vec3};

mod camera;
use camera::Camera;

mod obj;
use obj::Sphere;

struct HitRecord {
    t: f32,
    p: Vec3,
    normal: Vec3,
}

impl HitRecord {
    fn new(t: f32, p: Vec3, normal: Vec3) -> HitRecord {
        HitRecord {
            t: t,
            p: p,
            normal: normal,
        }
    }
}

trait Hittable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl Hittable for Sphere {
    // Solves a quadratic equation
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let a = dot(r.b, r.b);
        let b = 2.0 * dot(r.b, r.a - self.center);
        let c = dot(r.a - self.center, r.a - self.center) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            // Check smaller parameter
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t_min < t && t < t_max {
                let normal = (r.point_at_parameter(t) - self.center) / self.radius;
                return Some(HitRecord::new(t, r.point_at_parameter(t), normal));
            }

            // Check larger parameter
            let t = (-b + discriminant.sqrt()) / (2.0 * a);
            if t_min < t && t < t_max {
                let normal = (r.point_at_parameter(t) - self.center) / self.radius;
                return Some(HitRecord::new(t, r.point_at_parameter(t), normal));
            }
        }
        None
    }
}

fn color(r: Ray) -> Vec3 {
    let s = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let q = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    match s.hit(r, 0.0, 1.0) {
        Some(h) => {
            let normal = h.normal;
            return 0.5 * Vec3::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
        }
        None => (),
    }
    match q.hit(r, 0.0, std::f32::MAX) {
        Some(h) => {
            let normal = h.normal;
            return 0.5 * Vec3::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
        }
        None => (),
    }

    let unit_direction = r.direction().unit();
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
}

fn rand_float() -> f32 {
    StdRng::from_entropy().sample(Standard)
}

fn main() {
    let (nx, ny, ns) = (200, 100, 100);
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let cam = Camera::default();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::default();
            for _ in 0..ns {
                let u: f32 = (i as f32 + rand_float()) / nx as f32;
                let v: f32 = (j as f32 + rand_float()) / ny as f32;
                let r = cam.get_ray(u, v);
                col += color(r);
            }
            col /= ns as f32;

            let ir: u32 = (255.99 * col[0]) as u32;
            let ig: u32 = (255.99 * col[1]) as u32;
            let ib: u32 = (255.99 * col[2]) as u32;
            println!("{} {} {}", ir, ig, ib)
        }
    }
}
