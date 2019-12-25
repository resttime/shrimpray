use crate::vec3::Vec3;

#[derive(Default)]
pub struct Camera {
    origin: Vec3,
    ll_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
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
}

fn hi() {
    println!("Test");
}
