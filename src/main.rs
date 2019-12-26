use rand::Rng;

mod vec3;
use vec3::{Ray, Vec3};

mod camera;
use camera::Camera;

mod obj;
use obj::Sphere;

mod hit;
use hit::Hittable;

fn color(r: Ray, world: &Vec<&dyn Hittable>) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.001, std::f32::MAX) {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        return 0.5 * color(Ray::new(hit.p, target - hit.p), world);
    }

    let unit_direction = r.direction().unit();
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
}

fn rand_float() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen::<f32>()
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p =
            2.0 * Vec3::new(rand_float(), rand_float(), rand_float()) - Vec3::new(1.0, 1.0, 1.0);
        if p.mag() < 1.0 {
            return p;
        }
    }
}

fn main() {
    let (nx, ny, ns) = (200, 100, 100);
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let cam = Camera::default();
    let s = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let q = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    let world = vec![&s as &dyn Hittable, &q as &dyn Hittable];

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::default();
            for _ in 0..ns {
                let u: f32 = (i as f32 + rand_float()) / nx as f32;
                let v: f32 = (j as f32 + rand_float()) / ny as f32;
                let r = cam.get_ray(u, v);
                col += color(r, &world);
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
