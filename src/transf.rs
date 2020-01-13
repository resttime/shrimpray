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
    pub offset: Vec3,
    pub obj_ref: Rc<dyn Hittable>,
}

impl Translate {
    pub fn new(p: Rc<dyn Hittable>, displacement: Vec3) -> Self {
        Self {
            offset: displacement,
            obj_ref: p,
        }
    }
}
