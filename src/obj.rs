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
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}
