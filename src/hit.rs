use std::rc::Rc;

use crate::bvh::{surrounding_bbox, BvhNode, AABB};
use crate::material::Material;
use crate::obj::{MovingSphere, Sphere};
use crate::vec3::{dot, Ray, Vec3};

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub u: f32,
    pub v: f32,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    fn new(t: f32, p: Vec3, normal: Vec3, u: f32, v: f32, material: Rc<dyn Material>) -> HitRecord {
        HitRecord {
            t: t,
            p: p,
            normal: normal,
            u: u,
            v: v,
            material: material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
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
                let (u, v) = Sphere::get_sphere_uv(&normal);
                return Some(HitRecord::new(t, point, normal, u, v, Rc::clone(&self.material)));
            }

            // Check larger parameter
            let t = (-b + discriminant.sqrt()) / a;
            if t_min < t && t < t_max {
                let point = r.point_at_parameter(t);
                let normal = (point - self.center) / self.radius;
                let (u, v) = Sphere::get_sphere_uv(&normal);
                return Some(HitRecord::new(t, point, normal, u, v, Rc::clone(&self.material)));
            }
        }
        None
    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let bbox = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(bbox)
    }
}

impl Hittable for Vec<Rc<dyn Hittable>> {
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
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.is_empty() {
            return None;
        }

        let mut bbox: AABB;
        if let Some(new_bbox) = self[0].bounding_box(t0, t1) {
            bbox = new_bbox;
        } else {
            return None;
        }

        for obj in self.iter().skip(1) {
            match obj.bounding_box(t0, t1) {
                Some(new_bbox) => bbox = surrounding_bbox(bbox, new_bbox),
                None => {
                    return None;
                }
            }
        }
        Some(bbox)
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
                let (u, v) = Sphere::get_sphere_uv(&normal);
                return Some(HitRecord::new(t, point, normal, u, v, Rc::clone(&self.material)));
            }

            // Check larger parameter
            let t = (-b + discriminant.sqrt()) / a;
            if t_min < t && t < t_max {
                let point = r.point_at_parameter(t);
                let normal = (point - self.center(r.time())) / self.radius;
                let (u, v) = Sphere::get_sphere_uv(&normal);
                return Some(HitRecord::new(t, point, normal, u, v, Rc::clone(&self.material)));
            }
        }
        None
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let bbox0 = AABB::new(
            self.center(t0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let bbox1 = AABB::new(
            self.center(t1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(surrounding_bbox(bbox0, bbox1))
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match (&self.left, &self.right) {
            (Some(left), Some(right)) => {
                if self.bbox.hit(&r, t_min, t_max) {
                    match (left.hit(r, t_min, t_max), right.hit(r, t_min, t_max)) {
                        (Some(lhit), Some(rhit)) => {
                            if lhit.t < rhit.t {
                                return Some(lhit);
                            } else {
                                return Some(rhit);
                            }
                        }
                        (Some(lhit), None) => return Some(lhit),
                        (None, Some(rhit)) => return Some(rhit),
                        (None, None) => (),
                    }
                }
            }
            (_, _) => (),
        }
        None
    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(self.bbox.min(), self.bbox.max()))
    }
}
