mod vec3;
use vec3::{Vec3, dot};

#[derive(Copy, Clone)]
struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    fn direction(&self) -> Vec3 {
        self.b-self.a
    }
    fn new(a: Vec3, b: Vec3) -> Ray {
        Ray {a: a, b: b}
    }
}

fn hit_sphere(r: Ray) -> bool {
    let center = Vec3::new(0.0, 0.0, -1.0);
    let radius: f32 = 0.5;

    let a = dot(r.b, r.b);
    let b = 2.0*dot(r.b, r.a-center);
    let c = dot(r.a-center, r.a-center) - radius*radius;

    let discriminant = b*b-4.0*a*c;

    discriminant >= 0.0
}

fn color(r: Ray) -> Vec3 {
    let unit_direction = r.direction().unit();
    let t: f32 = 0.5*(unit_direction.y() + 1.0);
    if hit_sphere(r) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    (1.0-t)*Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let (nx, ny): (u32, u32) = (200, 100);
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let ll_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u: f32 = i as f32 / nx as f32;
            let v: f32 = j as f32 / ny as f32;
            let r = Ray::new(origin, ll_corner + u*horizontal + v*vertical);
            let col = color(r);
            let ir: u32 = (255.99*col[0]) as u32;
            let ig: u32 = (255.99*col[1]) as u32;
            let ib: u32 = (255.99*col[2]) as u32;
            println!("{} {} {}", ir, ig, ib)
        }
    }
}
