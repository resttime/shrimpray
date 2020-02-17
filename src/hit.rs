use std::sync::Arc;

use crate::bvh::*;
use crate::material::Material;
use crate::obj::*;
use crate::transf::*;
use crate::util::*;
use crate::vec3::*;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub u: f32,
    pub v: f32,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    fn new(t: f32, p: Vec3, normal: Vec3, u: f32, v: f32, material: Arc<dyn Material>) -> HitRecord {
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

pub trait Hittable : Sync + Send {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
    fn pdf_value(&self, _o: &Vec3, _v: &Vec3) -> f32 { 0.0 }
    fn random(&self, _o: &Vec3) -> Vec3 { Vec3::new(1.0, 0.0, 0.0) }
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
                return Some(HitRecord::new(
                    t,
                    point,
                    normal,
                    u,
                    v,
                    Arc::clone(&self.material),
                ));
            }

            // Check larger parameter
            let t = (-b + discriminant.sqrt()) / a;
            if t_min < t && t < t_max {
                let point = r.point_at_parameter(t);
                let normal = (point - self.center) / self.radius;
                let (u, v) = Sphere::get_sphere_uv(&normal);
                return Some(HitRecord::new(
                    t,
                    point,
                    normal,
                    u,
                    v,
                    Arc::clone(&self.material),
                ));
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

impl Hittable for Vec<Arc<dyn Hittable>> {
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
                return Some(HitRecord::new(
                    t,
                    point,
                    normal,
                    u,
                    v,
                    Arc::clone(&self.material),
                ));
            }

            // Check larger parameter
            let t = (-b + discriminant.sqrt()) / a;
            if t_min < t && t < t_max {
                let point = r.point_at_parameter(t);
                let normal = (point - self.center(r.time())) / self.radius;
                let (u, v) = Sphere::get_sphere_uv(&normal);
                return Some(HitRecord::new(
                    t,
                    point,
                    normal,
                    u,
                    v,
                    Arc::clone(&self.material),
                ));
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

impl Hittable for XYRect {
    fn hit(&self, r: Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t0 || t > t1 {
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let p = r.point_at_parameter(t);
        let normal = Vec3::new(0.0, 0.0, 1.0);
        Some(HitRecord::new(t, p, normal, u, v, self.material.clone()))
    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

impl Hittable for XZRect {
    fn hit(&self, r: Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t0 || t > t1 {
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let p = r.point_at_parameter(t);
        let normal = Vec3::new(0.0, 1.0, 0.0);
        Some(HitRecord::new(t, p, normal, u, v, self.material.clone()))
    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f32 {
        if let Some(hit) = self.hit(Ray::new(*o, *v, 0.0), 0.001, std::f32::MAX) {
            let area = (self.x1 - self.x0) * (self.z1 - self.z0);
            let dist_sqrd = hit.t * hit.t * v.mag().powi(2);
            let cosine = (dot(*v, hit.normal) / v.mag()).abs();
            return dist_sqrd / (cosine * area);
        }
        0.0
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        let random_point = Vec3::new(self.x0 + rand_float() * (self.x1-self.x0),
                                     self.k,
                                     self.z0 + rand_float()*(self.z1-self.z0));
        random_point - *o
    }
}

impl Hittable for YZRect {
    fn hit(&self, r: Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t0 || t > t1 {
            return None;
        }
        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let p = r.point_at_parameter(t);
        let normal = Vec3::new(1.0, 0.0, 0.0);
        Some(HitRecord::new(t, p, normal, u, v, self.material.clone()))
    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}

impl Hittable for FlipNormals {
    fn hit(&self, r: Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        if let Some(mut hit) = self.obj_ref.hit(r, t0, t1) {
            hit.normal *= -1.0;
            return Some(hit);
        }
        None
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.obj_ref.bounding_box(t0, t1)
    }
}

impl Hittable for BoxShape {
    fn hit(&self, r: Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        self.faces.hit(r, t0, t1)
    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(self.pmin, self.pmax))
    }
}

impl Hittable for Translate {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_ray = Ray::new(r.origin() - self.offset, r.direction(), r.time());

        if let Some(mut hit) = self.obj_ref.hit(moved_ray, t_min, t_max) {
            hit.p += self.offset;
            return Some(hit);
        }
        None
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if let Some(bbox) = self.obj_ref.bounding_box(t0, t1) {
            return Some(AABB::new(
                bbox.min() + self.offset,
                bbox.max() + self.offset,
            ));
        }
        None
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];
        let rotate_r = Ray::new(origin, direction, r.time());
        if let Some(mut hit) = self.obj_ref.hit(rotate_r, t_min, t_max) {
            let mut p = hit.p;
            let mut normal = hit.normal;
            p[0] = self.cos_theta * hit.p[0] + self.sin_theta * hit.p[2];
            p[2] = -self.sin_theta * hit.p[0] + self.cos_theta * hit.p[2];
            normal[0] = self.cos_theta * hit.normal[0] + self.sin_theta * hit.normal[2];
            normal[2] = -self.sin_theta * hit.normal[0] + self.cos_theta * hit.normal[2];
            hit.p = p;
            hit.normal = normal;
            return Some(hit);
        }
        None
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if let Some(bbox) = self.obj_ref.bounding_box(t0, t1) {
            let mut max = Vec3::new(
                std::f32::NEG_INFINITY,
                std::f32::NEG_INFINITY,
                std::f32::NEG_INFINITY,
            );
            let mut min = Vec3::new(std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY);
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f32 * bbox.max().x() + (1.0 - i as f32) * bbox.min().x();
                        let y = j as f32 * bbox.max().y() + (1.0 - j as f32) * bbox.min().y();
                        let z = k as f32 * bbox.max().z() + (1.0 - k as f32) * bbox.min().z();

                        let new_x = self.cos_theta * x + self.sin_theta * z;
                        let new_z = -self.sin_theta * x + self.cos_theta * z;
                        let tester = Vec3::new(new_x, y, new_z);
                        for c in 0..3 {
                            if tester[c as u32] > max[c] {
                                max[c] = tester[c];
                            }
                            if tester[c as u32] < max[c] {
                                min[c] = tester[c];
                            }
                        }
                    }
                }
            }
            return Some(AABB::new(min, max));
        }
        None
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(mut hit1) = self
            .boundary
            .hit(r, std::f32::NEG_INFINITY, std::f32::INFINITY)
        {
            if let Some(mut hit2) = self.boundary.hit(r, hit1.t + 0.0001, std::f32::INFINITY) {
                if hit1.t < t_min {
                    hit1.t = t_min;
                }
                if hit2.t > t_max {
                    hit2.t = t_max;
                }
                if hit1.t >= hit2.t {
                    return None;
                }
                if hit1.t < 0.0 {
                    hit1.t = 0.0;
                }
                let dist_in_boundary = (hit2.t - hit1.t) * r.direction().mag();
                let hit_dist = -(1.0 / self.density) * rand_float().ln();
                if hit_dist < dist_in_boundary {
                    let t = hit1.t + hit_dist / r.direction().mag();
                    let p = r.point_at_parameter(t);
                    return Some(HitRecord::new(
                        t,
                        p,
                        Vec3::new(1.0, 0.0, 0.0),
                        0.0,
                        0.0,
                        self.phase_function.clone(),
                    ));
                }
            }
        }
        None
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}
