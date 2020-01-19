use crate::perlin::Perlin;
use crate::vec3::Vec3;

pub trait Texture : Sync + Send {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}

pub struct ConstantTexture {
    color: Vec3,
}

impl ConstantTexture {
    pub fn new(c: Vec3) -> Self {
        Self { color: c }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        self.color
    }
}

pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(t0: Box<dyn Texture>, t1: Box<dyn Texture>) -> Self {
        Self { odd: t0, even: t1 }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    scale: f32,
    noise: Perlin,
}

impl NoiseTexture {
    pub fn new(scale: f32, noise: Perlin) -> Self {
        Self {
            scale: scale,
            noise: noise,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: &Vec3) -> Vec3 {
        // Vec3::new(1.0, 1.0, 1.0)
        //     * 0.50
        //     * (1.0 + (self.scale * p.x() + 10.0 * self.noise.turb(p, 7)).sin())
        let scaled_p = Vec3::new(p.x(), p.y(), p.z()) * self.scale;
        Vec3::new(1.0, 1.0, 1.0)
            * 0.50
            * (1.0 + (self.scale * p.x() + 5.0 * self.noise.turb(&scaled_p, 7)).sin())
    }
}

pub struct ImageTexture {
    data: Vec<u8>,
    nx: i32,
    ny: i32,
}

impl ImageTexture {
    pub fn new(pixels: Vec<u8>, a: i32, b: i32) -> Self {
        Self{ data: pixels, nx: a, ny: b }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: &Vec3) -> Vec3 {
        let mut i = (u * self.nx as f32) as i32;
        let mut j = ((1.0-v) * self.ny as f32 - 0.001) as i32;
        if i < 0 { i = 0; }
        if j < 0 { j = 0; }
        if i > (self.nx - 1) {  i = self.nx - 1; }
        if j > (self.ny - 1) {  j = self.ny - 1; }
        let r = self.data[3*i as usize + (3*self.nx*j) as usize] as f32 / 255.0;
        let g = self.data[3*i as usize + (3*self.nx*j + 1) as usize] as f32 / 255.0;
        let b = self.data[3*i as usize + (3*self.nx*j + 2) as usize] as f32 / 255.0;
        Vec3::new(r, g, b)
    }
}
