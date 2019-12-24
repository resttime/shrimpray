struct vec3 {
    e1: f32,
    e2: f32,
    e3: f32,
}

fn main() {
    let (nx, ny): (u32, u32) = (200, 100);
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");
    for j in (0..ny).rev() {
        for i in 0..nx {
            let r: f32 = i as f32 / nx as f32;
            let g: f32 = j as f32 / ny as f32;
            let b: f32 = 0.2;
            let ir: u32 = (255.99*r) as u32;
            let ig: u32 = (255.99*g) as u32;
            let ib: u32 = (255.99*b) as u32;
            println!("{} {} {}", ir, ig, ib)
        }
    }
}
