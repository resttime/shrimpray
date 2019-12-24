use std::ops::Index;

struct Vec3 {
    e0: f32,
    e1: f32,
    e2: f32,
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

struct Ray {
    a: Vec3,
    b: Vec3,
}

fn main() {
    let (nx, ny): (u32, u32) = (200, 100);
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");
    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Vec3 {e0: i as f32 / nx as f32,
                            e1: j as f32 / ny as f32,
                            e2: 0.2};
            let ir: u32 = (255.99*col[0]) as u32;
            let ig: u32 = (255.99*col[1]) as u32;
            let ib: u32 = (255.99*col[2]) as u32;
            println!("{} {} {}", ir, ig, ib)
        }
    }
}
