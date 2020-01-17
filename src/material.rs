use std::rc::Rc;

use crate::hit::HitRecord;
use crate::texture::Texture;
use crate::util::*;
use crate::vec3::{dot, reflect, refract, Ray, Vec3};

pub trait Material {
    // -> Option<SCATTERED: Ray, ATTENUATION: Vec3>
    fn scatter(&self, ray_in: Ray, hit: &HitRecord) -> Option<(Ray, Vec3)>;
    fn emitted(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

// Diffuse
pub struct Lambertian {
    albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(a: Rc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p, ray_in.time());
        let attenuation = self.albedo.value(hit.u, hit.v, &hit.p);

        Some((scattered, attenuation))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(a: Vec3, f: f32) -> Self {
        let mut clamped_f = 1.0;
        if f < 1.0 {
            clamped_f = f;
        }
        Self {
            albedo: a,
            fuzz: clamped_f,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected: Vec3 = reflect(ray_in.direction().unit(), hit.normal);
        let scattered = Ray::new(
            hit.p,
            reflected + self.fuzz * random_in_unit_sphere(),
            ray_in.time(),
        );
        let attenuation = self.albedo;

        if dot(scattered.direction(), hit.normal) > 0.0 {
            return Some((scattered, attenuation));
        }
        None
    }
}

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ri: f32) -> Self {
        Self { ref_idx: ri }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let outward_normal: Vec3;
        let ni_over_nt: f32;
        let cosine: f32;

        if dot(ray_in.direction(), hit.normal) > 0.0 {
            outward_normal = -1.0 * hit.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * dot(ray_in.direction(), hit.normal) / ray_in.direction().mag();
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -1.0 * dot(ray_in.direction(), hit.normal) / ray_in.direction().mag();
        }

        let reflected = reflect(ray_in.direction(), hit.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        if let Some(refracted) = refract(ray_in.direction(), outward_normal, ni_over_nt) {
            let refract_prob = 1.0 - schlick(cosine, self.ref_idx);
            if rand_float() < refract_prob {
                return Some((Ray::new(hit.p, refracted, ray_in.time()), attenuation));
            }
        }
        Some((Ray::new(hit.p, reflected, ray_in.time()), attenuation))
    }
}

pub struct DiffuseLight {
    emit: Rc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(a: Rc<dyn Texture>) -> Self {
        Self { emit: a }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray_in: Ray, _hit: &HitRecord) -> Option<(Ray, Vec3)> {
        None
    }
    fn emitted(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}

pub struct Isotropic {
    albedo: Rc<dyn Texture>,
}

impl Isotropic {
    pub fn new(a: Rc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _ray_in: Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let scattered = Ray::new(hit.p, random_in_unit_sphere(), hit.t);
        let attenuation = self.albedo.value(hit.u, hit.v, &hit.p);
        Some((scattered, attenuation))
    }
}
