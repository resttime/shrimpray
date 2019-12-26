use crate::obj::Sphere;
use crate::vec3::{dot, Ray, Vec3};

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
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

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl Hittable for Sphere {
    // Solves a quadratic equation
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let a = dot(r.b, r.b);
        let b = dot(r.b, r.a - self.center);
        let c = dot(r.a - self.center, r.a - self.center) - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            // Check smaller parameter
            let t = (-b - discriminant.sqrt()) / a;
            if t_min < t && t < t_max {
                let point = r.point_at_parameter(t);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord::new(t, point, normal));
            }

            // Check larger parameter
            let t = (-b + discriminant.sqrt()) / a;
            if t_min < t && t < t_max {
                let point = r.point_at_parameter(t);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord::new(t, point, normal));
            }
        }
        None
    }
}

impl Hittable for Vec<&dyn Hittable> {
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
