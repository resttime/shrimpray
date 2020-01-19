use std::sync::Arc;

use image::GenericImageView;

use crate::bvh::*;
use crate::hit::*;
use crate::material::*;
use crate::obj::*;
use crate::perlin::Perlin;
use crate::texture::*;
use crate::transf::*;
use crate::util::*;
use crate::vec3::*;

pub fn regular_scene() -> Vec<Arc<dyn Hittable>> {
    let world: Vec<Arc<dyn Hittable>> = vec![
        Arc::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
                0.1, 0.2, 0.5,
            ))))),
        )),
        Arc::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
                0.8, 0.8, 0.0,
            ))))),
        )),
        Arc::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)),
        )),
        Arc::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Arc::new(Dielectric::new(1.5)),
        )),
        Arc::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            -0.45,
            Arc::new(Dielectric::new(1.5)),
        )),
    ];
    world
}

pub fn random_scene() -> Vec<Arc<dyn Hittable>> {
    let mut scene: Vec<Arc<dyn Hittable>> = Vec::new();
    let checker = Arc::new(CheckerTexture::new(
        Box::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
        Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
    ));
    scene.push(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, -1.0),
        1000.0,
        Arc::new(Lambertian::new(checker)),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_float();
            let center = Vec3::new(
                a as f32 + 0.9 * rand_float(),
                0.2,
                b as f32 + 0.9 * rand_float(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).mag() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    scene.push(Arc::new(MovingSphere::new(
                        center,
                        center + Vec3::new(0.0, 0.5 * rand_float(), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
                            rand_float() * rand_float(),
                            rand_float() * rand_float(),
                            rand_float() * rand_float(),
                        ))))),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    scene.push(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + rand_float()),
                                0.5 * (1.0 + rand_float()),
                                0.5 * (1.0 + rand_float()),
                            ),
                            0.5 * rand_float(),
                        )),
                    )));
                } else {
                    // glass
                    scene.push(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }
    scene.push(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    scene.push(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
            0.4, 0.2, 0.1,
        ))))),
    )));
    scene.push(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));
    scene
}

pub fn two_spheres_scene() -> Vec<Arc<dyn Hittable>> {
    let mut scene: Vec<Arc<dyn Hittable>> = Vec::new();
    let checker = Arc::new(CheckerTexture::new(
        Box::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
        Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
    ));
    let checker2 = Arc::new(CheckerTexture::new(
        Box::new(ConstantTexture::new(Vec3::new(0.1, 0.2, 0.3))),
        Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
    ));
    scene.push(Arc::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker)),
    )));
    scene.push(Arc::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker2)),
    )));
    scene
}

pub fn two_perlin_spheres_scene() -> Vec<Arc<dyn Hittable>> {
    let mut scene: Vec<Arc<dyn Hittable>> = Vec::new();
    let perlin_texture = Arc::new(NoiseTexture::new(4.0, Perlin::new()));
    scene.push(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(perlin_texture.clone())),
    )));
    scene.push(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(perlin_texture)),
    )));
    scene
}

pub fn earth_scene() -> Vec<Arc<dyn Hittable>> {
    let mut scene: Vec<Arc<dyn Hittable>> = Vec::new();
    let perlin_texture = Arc::new(NoiseTexture::new(4.0, Perlin::new()));
    scene.push(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(perlin_texture)),
    )));

    let img = image::open("texture/earthmap.jpg").unwrap();
    let (nx, ny) = img.dimensions();
    let data = img.raw_pixels();
    let image_texture = Arc::new(ImageTexture::new(data, nx as i32, ny as i32));
    scene.push(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(image_texture)),
    )));
    scene
}

pub fn simple_light() -> Vec<Arc<dyn Hittable>> {
    let perlin_texture = Arc::new(NoiseTexture::new(4.0, Perlin::new()));
    let mut scene: Vec<Arc<dyn Hittable>> = Vec::new();
    scene.push(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(perlin_texture.clone())),
    )));
    scene.push(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(perlin_texture)),
    )));

    let constant_texture = Arc::new(ConstantTexture::new(Vec3::new(4.0, 4.0, 4.0)));
    scene.push(Arc::new(Sphere::new(
        Vec3::new(0.0, 7.0, 0.0),
        2.0,
        Arc::new(DiffuseLight::new(constant_texture.clone())),
    )));
    scene.push(Arc::new(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        Arc::new(DiffuseLight::new(constant_texture.clone())),
    )));
    scene
}

pub fn cornell_box() -> Vec<Arc<dyn Hittable>> {
    let mut scene: Vec<Arc<dyn Hittable>> = Vec::new();

    let red = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
        0.65, 0.05, 0.05,
    )))));
    let white = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
        0.73, 0.73, 0.73,
    )))));
    let green = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
        0.12, 0.45, 0.15,
    )))));
    let light = Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(Vec3::new(
        15.0, 15.0, 15.0,
    )))));

    scene.push(Arc::new(FlipNormals::new(Arc::new(YZRect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, green,
    )))));
    scene.push(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    scene.push(Arc::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    scene.push(Arc::new(FlipNormals::new(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )))));
    scene.push(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    scene.push(Arc::new(FlipNormals::new(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )))));

    let small_box = Arc::new(BoxShape::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let tall_box = Arc::new(BoxShape::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    scene.push(Arc::new(Translate::new(
        Arc::new(RotateY::new(small_box, -18.0)),
        Vec3::new(130.0, 0.0, 65.0),
    )));
    scene.push(Arc::new(Translate::new(
        Arc::new(RotateY::new(tall_box, 15.0)),
        Vec3::new(265.0, 0.0, 295.0),
    )));

    scene
}

pub fn cornell_smoke_scene() -> Vec<Arc<dyn Hittable>> {
    let mut scene: Vec<Arc<dyn Hittable>> = Vec::new();
    let red = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
        0.65, 0.05, 0.05,
    )))));
    let white = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
        0.73, 0.73, 0.73,
    )))));
    let green = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
        0.12, 0.45, 0.15,
    )))));
    let light = Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(Vec3::new(
        7.0, 7.0, 7.0,
    )))));

    scene.push(Arc::new(FlipNormals::new(Arc::new(YZRect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, green,
    )))));
    scene.push(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    scene.push(Arc::new(XZRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    scene.push(Arc::new(FlipNormals::new(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )))));
    scene.push(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    scene.push(Arc::new(FlipNormals::new(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )))));
    let tall_box = Arc::new(BoxShape::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let tall_box = Arc::new(RotateY::new(tall_box, 15.0));
    let tall_box = Arc::new(Translate::new(tall_box, Vec3::new(265.0, 0.0, 295.0)));
    let small_box = Arc::new(BoxShape::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let small_box = Arc::new(RotateY::new(small_box, -18.0));
    let small_box = Arc::new(Translate::new(small_box, Vec3::new(130.0, 0.0, 65.0)));
    scene.push(Arc::new(ConstantMedium::new(
        tall_box,
        0.01,
        Arc::new(ConstantTexture::new(Vec3::new(0.0, 0.0, 0.0))),
    )));
    scene.push(Arc::new(ConstantMedium::new(
        small_box,
        0.01,
        Arc::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))),
    )));
    scene
}

pub fn final_scene() -> Vec<Arc<dyn Hittable>> {
    // Create scene vector
    let mut scene: Vec<Arc<dyn Hittable>> = Vec::new();

    // Create and add a ground of boxes
    let mut boxes1: Vec<Arc<dyn Hittable>> = Vec::new();
    let ground = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
        0.48, 0.83, 0.53,
    )))));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rand_float() * 100.0 + 1.0;
            let z1 = z0 + w;
            boxes1.push(Arc::new(BoxShape::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }
    scene.push(Arc::new(BvhNode::new(&mut boxes1, 0.0, 1.0)));

    // Create and add lighting to scene
    let light = Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(Vec3::new(
        7.0, 7.0, 7.0,
    )))));
    scene.push(Arc::new(XZRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    // Add moving sphere
    let center = Vec3::new(400.0, 400.0, 200.0);
    scene.push(Arc::new(MovingSphere::new(
        center,
        center + Vec3::new(30.0, 0.0, 0.0),
        0.0,
        1.0,
        50.0,
        Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
            0.7, 0.3, 0.1,
        ))))),
    )));

    // Add dielectric and metal sphere
    scene.push(Arc::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    scene.push(Arc::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 10.0)),
    )));

    // Add boundary
    let mut boundary = Arc::new(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    scene.push(boundary.clone());

    // Add medium in boundary
    scene.push(Arc::new(ConstantMedium::new(
        boundary.clone(),
        0.2,
        Arc::new(ConstantTexture::new(Vec3::new(0.2, 0.4, 0.9))),
    )));

    // Add new medium in different boundary
    boundary = Arc::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    scene.push(Arc::new(ConstantMedium::new(
        boundary.clone(),
        0.0001,
        Arc::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))),
    )));

    // Add image textured sphere
    let img = image::open("texture/earthmap.jpg").unwrap();
    let (nx, ny) = img.dimensions();
    let data = img.raw_pixels();
    let image_texture = Arc::new(ImageTexture::new(data, nx as i32, ny as i32));
    let image_mat = Arc::new(Lambertian::new(image_texture));
    scene.push(Arc::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        image_mat,
    )));

    // Add perlin textured sphere
    let perlin_texture = Arc::new(NoiseTexture::new(0.1, Perlin::new()));
    scene.push(Arc::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new(perlin_texture.clone())),
    )));

    // Add rotated "box" of spheres
    let mut box_of_spheres: Vec<Arc<dyn Hittable>> = Vec::new();
    let white = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
        0.73, 0.73, 0.73,
    )))));
    for _ in 0..1000 {
        box_of_spheres.push(Arc::new(Sphere::new(
            Vec3::new(
                rand_float() * 165.0,
                rand_float() * 165.0,
                rand_float() * 165.0,
            ),
            10.0,
            white.clone(),
        )));
    }
    scene.push(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::new(&mut box_of_spheres, 0.0, 1.0)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    // All done, return the scene!
    scene
}
