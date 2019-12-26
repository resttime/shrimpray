use crate::hit::HitRecord;
use crate::util::*;
use crate::vec3::{reflect, Ray, Vec3};

pub trait Material {
    fn scatter(&self, ray_in: Ray, hit: &HitRecord) -> Option<(Ray, Vec3)>;
}

// Diffuse
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(a: Vec3) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p);
        let attenuation = self.albedo;

        Some((scattered, attenuation))
    }
}

pub struct Metal {
    albedo: Vec3,
}

impl Material for Metal {
    fn scatter(&self, ray_in: Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected: Vec3 = reflect(ray_in.direction().unit(), hit.normal);
        let scattered = Ray::new(hit.p, reflected);
        let attenuation = self.albedo;

        Some((scattered, attenuation))
    }
}
