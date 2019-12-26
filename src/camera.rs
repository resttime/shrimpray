use crate::vec3::{cross, Ray, Vec3};

pub struct Camera {
    origin: Vec3,
    ll_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            origin: Vec3::new(0.0, 0.0, 0.0),
            ll_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
        }
    }
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> Self {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = lookfrom;

        let w = (lookfrom - lookat).unit();
        let u = cross(vup, w);
        let v = cross(w, u);

        let ll_corner = origin - half_width*u - half_height*v - w;
        let horizontal = 2.0*half_width*u;
        let vertical = 2.0*half_height*v;

        Self {
            origin: origin,
            ll_corner: ll_corner,
            horizontal: horizontal,
            vertical: vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.ll_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
