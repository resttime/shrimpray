use crate::util::*;
use crate::vec3::{cross, Ray, Vec3};

#[derive(Default)]
#[allow(dead_code)]
pub struct Camera {
    origin: Vec3,
    ll_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3,
               vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = lookfrom;

        let w = (lookfrom - lookat).unit();
        let u = cross(vup, w).unit();
        let v = cross(w, u);

        let ll_corner = origin - half_width*focus_dist*u - half_height*focus_dist*v - focus_dist*w;
        let horizontal = 2.0*half_width*focus_dist*u;
        let vertical = 2.0*half_height*focus_dist*v;

        Self {
            origin: origin,
            ll_corner: ll_corner,
            horizontal: horizontal,
            vertical: vertical,
            u: u,
            v: v,
            w: w,
            lens_radius: lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self. u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.ll_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
