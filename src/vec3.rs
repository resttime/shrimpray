use std::ops::*;

#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3 {
    pub e0: f32,
    pub e1: f32,
    pub e2: f32,
}

impl Index<u32> for Vec3 {
    type Output = f32;
    fn index(&self, i: u32) -> &Self::Output {
        match i {
            0 => &self.e0,
            1 => &self.e1,
            2 => &self.e2,
            _ => panic!(),
        }
    }
}

impl IndexMut<u32> for Vec3 {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        match index {
            0 => &mut self.e0,
            1 => &mut self.e1,
            2 => &mut self.e2,
            _ => panic!(),
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            e0: self.e0 + other.e0,
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e0 += other.e0;
        self.e1 += other.e1;
        self.e2 += other.e2;
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            e0: self.e0 - other.e0,
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            e0: self.e0 * rhs.e0,
            e1: self.e1 * rhs.e1,
            e2: self.e2 * rhs.e2,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            e0: self.e0 * rhs,
            e1: self.e1 * rhs,
            e2: self.e2 * rhs,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e0: rhs.e0 * self,
            e1: rhs.e1 * self,
            e2: rhs.e2 * self,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.e0 *= rhs;
        self.e1 *= rhs;
        self.e2 *= rhs;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            e0: self.e0 / rhs,
            e1: self.e1 / rhs,
            e2: self.e2 / rhs,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.e0 /= rhs;
        self.e1 /= rhs;
        self.e2 /= rhs;
    }
}

impl std::iter::Sum<Vec3> for Vec3 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Vec3>,
    {
        let mut res = Vec3::default();
        for v in iter {
            res += v;
        }
        res
    }
}

impl Vec3 {
    pub fn mag(&self) -> f32 {
        dot(*self, *self).sqrt()
    }
    pub fn mag_sqrd(&self) -> f32 {
        dot(*self, *self)
    }
    pub fn unit(&self) -> Vec3 {
        *self / self.mag()
    }
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 {
            e0: e0,
            e1: e1,
            e2: e2,
        }
    }
    pub fn x(&self) -> f32 {
        self.e0
    }
    pub fn y(&self) -> f32 {
        self.e1
    }
    pub fn z(&self) -> f32 {
        self.e2
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f32 {
    u.e0 * v.e0 + u.e1 * v.e1 + u.e2 * v.e2
}

// n is a unit vector that we are reflecting
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3::new(
        v1.e1 * v2.e2 - v1.e2 * v2.e1,
        v1.e2 * v2.e0 - v1.e0 * v2.e2,
        v1.e0 * v2.e1 - v1.e1 * v2.e0,
    )
}

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.unit();
    let dt = dot(uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        return Some(refracted);
    }

    None
}

#[derive(Copy, Clone, Default)]
pub struct Ray {
    pub a: Vec3,
    pub b: Vec3,
    pub time: f32,
}

impl Ray {
    pub fn direction(&self) -> Vec3 {
        self.b
    }
    pub fn origin(&self) -> Vec3 {
        self.a
    }
    pub fn time(&self) -> f32 {
        self.time
    }
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.a + t * self.b
    }
    pub fn new(a: Vec3, b: Vec3, t: f32) -> Ray {
        Ray {
            a: a,
            b: b,
            time: t,
        }
    }
}

pub struct Onb {
    pub axis: Vec<Vec3>,
}

impl Onb {
    pub fn new() -> Self {
        let uvw = vec![
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ];
        Onb { axis: uvw }
    }
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }
    pub fn local_vector(&self, a: &Vec3) -> Vec3 {
        a.x() * self.u() + a.y() * self.v() + a.z() * self.w()
    }
    pub fn local_coordinates(&self, a: f32, b: f32, c: f32) -> Vec3 {
        a * self.u() + b * self.v() + c * self.w()
    }
    pub fn build_from_w(&mut self, n: &Vec3) {
        self.axis[2] = n.unit();
        let a: Vec3;
        if self.w().x().abs() > 0.9 {
            a = Vec3::new(0.0, 1.0, 0.0);
        } else {
            a = Vec3::new(1.0, 0.0, 0.0);
        }
        self.axis[1] = cross(self.w(), a).unit();
        self.axis[0] = cross(self.w(), self.v());
    }
}

impl Index<u32> for Onb {
    type Output = Vec3;
    fn index(&self, i: u32) -> &Self::Output {
        match i {
            0 => &self.axis[0],
            1 => &self.axis[1],
            2 => &self.axis[2],
            _ => panic!(),
        }
    }
}

impl IndexMut<u32> for Onb {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        match index {
            0 => &mut self.axis[0],
            1 => &mut self.axis[1],
            2 => &mut self.axis[2],
            _ => panic!(),
        }
    }
}
