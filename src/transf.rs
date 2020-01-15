use std::rc::Rc;

use crate::hit::Hittable;
use crate::vec3::Vec3;

pub struct FlipNormals {
    pub obj_ref: Rc<dyn Hittable>,
}

impl FlipNormals {
    pub fn new(obj_ref: Rc<dyn Hittable>) -> Self {
        Self { obj_ref: obj_ref }
    }
}

pub struct Translate {
    pub obj_ref: Rc<dyn Hittable>,
    pub offset: Vec3,
}

impl Translate {
    pub fn new(obj_ref: Rc<dyn Hittable>, offset: Vec3) -> Self {
        Self {
            offset: offset,
            obj_ref: obj_ref,
        }
    }
}

pub struct RotateY {
    pub obj_ref: Rc<dyn Hittable>,
    pub sin_theta: f32,
    pub cos_theta: f32,
}

impl RotateY {
    pub fn new(obj_ref: Rc<dyn Hittable>, angle: f32) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        Self {
            obj_ref: obj_ref,
            sin_theta: sin_theta,
            cos_theta: cos_theta,
        }
    }
}
