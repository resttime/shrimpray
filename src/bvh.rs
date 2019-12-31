use crate::hit::Hittable;
use crate::vec3::{Ray, Vec3};
use crate::util::*;

pub struct AABB {
    _min: Vec3,
    _max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self {
            _min: min,
            _max: max,
        }
    }
    pub fn min(&self) -> Vec3 {
        self._min
    }
    pub fn max(&self) -> Vec3 {
        self._max
    }
    pub fn hit(&self, r: &Ray, mut t_min: f32, mut t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let mut t0 = (self.min()[a] - r.origin()[a]) * inv_d;
            let mut t1 = (self.max()[a] - r.origin()[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let small = Vec3::new(
        box0.min().x().min(box1.min().x()),
        box0.min().y().min(box1.min().y()),
        box0.min().z().min(box1.min().z()),
    );
    let big = Vec3::new(
        box0.max().x().max(box1.max().x()),
        box0.max().y().max(box1.max().y()),
        box0.max().z().max(box1.max().z()),
    );
    AABB::new(small, big)
}

pub struct BvhNode {
    pub left: Box<dyn Hittable>,
    pub right: Box<dyn Hittable>,
    pub bbox: AABB,
}

impl BvhNode {
    // TODO Complete method
    pub fn new(list: &mut Vec<Box<dyn Hittable>>, time0: f32, time1: f32) {
        let axis = (3.0 * rand_float()) as u32;
        match axis {
            0 => {},
            1 => (),
            2 => (),
            _ => (),
        }
    }
}

pub fn box_compare_x(a: &dyn Hittable, b: &dyn Hittable) -> std::cmp::Ordering {
    match (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
        (Some(lbox), Some(rbox)) => {
            lbox.min().x().partial_cmp(&rbox.min().x()).unwrap()
        },
        (_, _) => panic!("Missing bounding box in a BvhNode contructor"),
    }
}

pub fn box_compare_y(a: &dyn Hittable, b: &dyn Hittable) -> std::cmp::Ordering {
    match (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
        (Some(lbox), Some(rbox)) => {
            lbox.min().y().partial_cmp(&rbox.min().y()).unwrap()
        },
        (_, _) => panic!("Missing bounding box in a BvhNode contructor"),
    }
}
pub fn box_compare_z(a: &dyn Hittable, b: &dyn Hittable) -> std::cmp::Ordering {
    match (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
        (Some(lbox), Some(rbox)) => {
            lbox.min().z().partial_cmp(&rbox.min().z()).unwrap()
        },
        (_, _) => panic!("Missing bounding box in a BvhNode contructor"),
    }
}
