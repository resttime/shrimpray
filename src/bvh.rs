use std::sync::Arc;

use crate::hit::Hittable;
use crate::util::*;
use crate::vec3::{Ray, Vec3};

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

pub fn surrounding_bbox(bbox0: AABB, bbox1: AABB) -> AABB {
    let small = Vec3::new(
        bbox0.min().x().min(bbox1.min().x()),
        bbox0.min().y().min(bbox1.min().y()),
        bbox0.min().z().min(bbox1.min().z()),
    );
    let big = Vec3::new(
        bbox0.max().x().max(bbox1.max().x()),
        bbox0.max().y().max(bbox1.max().y()),
        bbox0.max().z().max(bbox1.max().z()),
    );
    AABB::new(small, big)
}

pub struct BvhNode {
    pub left: Option<Arc<dyn Hittable>>,
    pub right: Option<Arc<dyn Hittable>>,
    pub bbox: AABB,
}

impl BvhNode {
    pub fn new(list: &mut [Arc<dyn Hittable>], time0: f32, time1: f32) -> Self {
        let axis = (3.0 * rand_float()) as u32;
        match axis {
            0 => {
                list.sort_by(box_compare_x);
            }
            1 => {
                list.sort_by(box_compare_y);
            }
            2 => {
                list.sort_by(box_compare_z);
            }
            _ => { panic!("There should've been an axis"); }
        }

        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>);
        let len = list.len();
        match len {
            1 => {
                left = list[0].clone();
                right = list[0].clone();
            }
            2 => {
                left = list[0].clone();
                right = list[1].clone();
            }
            _ => {
                left = Arc::new(BvhNode::new(&mut list[0..len / 2], time0, time1));
                right = Arc::new(BvhNode::new(&mut list[len / 2..], time0, time1));
            }
        }

        let l_bbox = left.bounding_box(time0, time1).expect("no bounding box");
        let r_bbox = right.bounding_box(time0, time1).expect("no bounding box");

        Self {
            left: Some(left),
            right: Some(right),
            bbox: surrounding_bbox(l_bbox, r_bbox),
        }
    }
}

pub fn box_compare_x(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
    match (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
        (Some(lbox), Some(rbox)) => lbox.min().x().partial_cmp(&rbox.min().x()).unwrap(),
        (_, _) => panic!("Missing bounding box in a BvhNode contructor"),
    }
}

pub fn box_compare_y(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
    match (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
        (Some(lbox), Some(rbox)) => lbox.min().y().partial_cmp(&rbox.min().y()).unwrap(),
        (_, _) => panic!("Missing bounding box in a BvhNode contructor"),
    }
}
pub fn box_compare_z(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
    match (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
        (Some(lbox), Some(rbox)) => lbox.min().z().partial_cmp(&rbox.min().z()).unwrap(),
        (_, _) => panic!("Missing bounding box in a BvhNode contructor"),
    }
}
