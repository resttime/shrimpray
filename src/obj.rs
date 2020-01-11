use std::rc::Rc;

use crate::material::Material;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
    pub fn get_sphere_uv(p: &Vec3) -> (f32, f32) {
        let phi = p.z().atan2(p.x());
        let theta = p.y().asin();
        let u = 1.0 - (phi + std::f32::consts::PI) / (2.0 * std::f32::consts::PI);
        let v = (theta + std::f32::consts::FRAC_PI_2) / std::f32::consts::PI;
        (u, v)
    }
}

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub material: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        t0: f32,
        t1: f32,
        radius: f32,
        material: Rc<dyn Material>,
    ) -> Self {
        Self {
            center0: center0,
            center1: center1,
            time0: t0,
            time1: t1,
            radius: radius,
            material: material,
        }
    }
    pub fn center(&self, time: f32) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

pub struct XYRect {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub material: Rc<dyn Material>,
}

impl XYRect {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Rc<dyn Material>) -> Self {
        Self {
            x0: x0,
            x1: x1,
            y0: y0,
            y1: y1,
            k: k,
            material: material,
        }
    }
}
