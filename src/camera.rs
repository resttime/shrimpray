use crate::vec3::{Ray, Vec3};

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
    pub fn new(origin: Vec3, ll_corner: Vec3, horizontal: Vec3, vertical: Vec3) -> Camera {
        Camera {
            origin: origin,
            ll_corner: ll_corner,
            horizontal: horizontal,
            vertical: vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.ll_corner + u * self.horizontal + v * self.vertical)
    }
}
