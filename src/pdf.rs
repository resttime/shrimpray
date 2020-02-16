use std::sync::Arc;

use crate::util::*;
use crate::vec3::*;
use crate::hit::*;

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f32;
    fn generate(&self) -> Vec3;
}

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        let mut uvw = Onb::new();
        uvw.build_from_w(w);
        CosinePdf { uvw: uvw }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f32 {
        let cosine = dot(direction.unit(), self.uvw.w());
        if cosine > 0.0 {
            return cosine / std::f32::consts::PI;
        }
        0.0
    }
    fn generate(&self) -> Vec3 {
        self.uvw.local_vector(&random_cosine_direction())
    }
}

pub struct HittablePdf {
    o: Vec3,
    obj_ref: Arc<dyn Hittable>,
}

impl HittablePdf {
    pub fn new(p: Arc<dyn Hittable>, origin: Vec3) -> Self {
        HittablePdf { o: origin, obj_ref: p }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: &Vec3) -> f32 {
        self.obj_ref.pdf_value(&self.o, &direction)
    }
    fn generate(&self) -> Vec3 {
        self.obj_ref.random(&self.o)
    }
}

pub struct MixturePdf {
    p: Vec<Box<dyn Pdf>>,
}

impl MixturePdf {
    pub fn new(p0: Box<dyn Pdf>, p1: Box<dyn Pdf>) -> Self {
        let p = vec![p0, p1];
        MixturePdf { p: p }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: &Vec3) -> f32 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }
    fn generate(&self, ) -> Vec3 {
        if rand_float() < 0.5 {
            return self.p[0].generate();
        }
        self.p[1].generate()
    }
}
