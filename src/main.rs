use rand::Rng;

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

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_t: f32 = t_max;
        let mut closest_hit: Option<HitRecord> = None;
        for obj in self {
            if let Some(hit) = obj.hit(r, t_min, closest_t) {
                closest_t = hit.t;
                closest_hit = Some(hit);
            }
        }
        closest_hit
    }
}

fn color(r: Ray, world: &Vec<Box<dyn Hittable>>) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.0, std::f32::MAX) {
        return 0.5 * Vec3::new(hit.normal.x()+1.0, hit.normal.y()+1.0, hit.normal.z()+1.0);
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
    let world = vec![Box::new(s) as Box<dyn Hittable>, Box::new(q) as Box<dyn Hittable>];

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

            let ir: u32 = (255.99 * col[0]) as u32;
            let ig: u32 = (255.99 * col[1]) as u32;
            let ib: u32 = (255.99 * col[2]) as u32;
            println!("{} {} {}", ir, ig, ib)
        }
    }
}
