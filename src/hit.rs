use std::rc::Rc;

use crate::material::{Material};
use crate::obj::{MovingSphere, Sphere};
use crate::vec3::{dot, Ray, Vec3};

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    fn new(t: f32, p: Vec3, normal: Vec3, material: Rc<dyn Material>) -> HitRecord {
        HitRecord {
            t: t,
            p: p,
            normal: normal,
            material: material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl Hittable for Sphere {
    // Solves a quadratic equation
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = dot(r.b, r.b);
        let b = dot(oc, r.b);
        let c = dot(oc, oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            // Check smaller parameter
            let t = (-b - discriminant.sqrt()) / a;
            if t_min < t && t < t_max {
                let point = r.point_at_parameter(t);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord::new(t, point, normal, Rc::clone(&self.material)));
            }

            // Check larger parameter
            let t = (-b + discriminant.sqrt()) / a;
            if t_min < t && t < t_max {
                let point = r.point_at_parameter(t);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord::new(t, point, normal, Rc::clone(&self.material)));
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

impl Hittable for MovingSphere {
    // Solves a quadratic equation
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());
        let a = dot(r.b, r.b);
        let b = dot(oc, r.b);
        let c = dot(oc, oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            // Check smaller parameter
            let t = (-b - discriminant.sqrt()) / a;
            if t_min < t && t < t_max {
                let point = r.point_at_parameter(t);
                let normal = (point - self.center(r.time())) / self.radius;
                return Some(HitRecord::new(t, point, normal, Rc::clone(&self.material)));
            }

            // Check larger parameter
            let t = (-b + discriminant.sqrt()) / a;
            if t_min < t && t < t_max {
                let point = r.point_at_parameter(t);
                let normal = (point - self.center(r.time())) / self.radius;
                return Some(HitRecord::new(t, point, normal, Rc::clone(&self.material)));
            }
        }
        None
    }
}
