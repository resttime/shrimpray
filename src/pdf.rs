use crate::util::*;
use crate::vec3::*;

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
