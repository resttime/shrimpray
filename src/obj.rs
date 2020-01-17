use std::rc::Rc;

use crate::hit::*;
use crate::material::*;
use crate::transf::*;
use crate::texture::*;
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

pub struct XZRect {
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: Rc<dyn Material>,
}

impl XZRect {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: Rc<dyn Material>) -> Self {
        Self {
            x0: x0,
            x1: x1,
            z0: z0,
            z1: z1,
            k: k,
            material: material,
        }
    }
}

pub struct YZRect {
    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: Rc<dyn Material>,
}

impl YZRect {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: Rc<dyn Material>) -> Self {
        Self {
            y0: y0,
            y1: y1,
            z0: z0,
            z1: z1,
            k: k,
            material: material,
        }
    }
}

pub struct BoxShape {
    pub pmin: Vec3,
    pub pmax: Vec3,
    pub faces: Vec<Rc<dyn Hittable>>,
}

impl BoxShape {
    pub fn new(p0: Vec3, p1: Vec3, mat: Rc<dyn Material>) -> Self {
        let mut faces: Vec<Rc<dyn Hittable>> = Vec::new();
        faces.push(Rc::new(XYRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            mat.clone(),
        )));
        faces.push(Rc::new(FlipNormals::new(Rc::new(XYRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            mat.clone(),
        )))));

        faces.push(Rc::new(XZRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            mat.clone(),
        )));
        faces.push(Rc::new(FlipNormals::new(Rc::new(XZRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            mat.clone(),
        )))));

        faces.push(Rc::new(YZRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            mat.clone(),
        )));
        faces.push(Rc::new(FlipNormals::new(Rc::new(YZRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            mat.clone(),
        )))));
        Self {
            pmin: p0,
            pmax: p1,
            faces: faces,
        }
    }
}

pub struct ConstantMedium {
    pub boundary: Rc<dyn Hittable>,
    pub density: f32,
    pub phase_function: Rc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(b: Rc<dyn Hittable>, d: f32, a: Rc<dyn Texture>) -> Self {
        Self {
            boundary: b,
            density: d,
            phase_function: Rc::new(Isotropic::new(a)),
        }
    }
}
