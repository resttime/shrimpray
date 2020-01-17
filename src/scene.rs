use std::rc::Rc;

use image::GenericImageView;

use crate::hit::*;
use crate::material::*;
use crate::obj::*;
use crate::perlin::Perlin;
use crate::texture::*;
use crate::transf::*;
use crate::util::*;
use crate::vec3::*;

pub fn regular_scene() -> Vec<Rc<dyn Hittable>> {
    let world: Vec<Rc<dyn Hittable>> = vec![
        Rc::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
                0.1, 0.2, 0.5,
            ))))),
        )),
        Rc::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
                0.8, 0.8, 0.0,
            ))))),
        )),
        Rc::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)),
        )),
        Rc::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Rc::new(Dielectric::new(1.5)),
        )),
        Rc::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            -0.45,
            Rc::new(Dielectric::new(1.5)),
        )),
    ];
    world
}

pub fn random_scene() -> Vec<Rc<dyn Hittable>> {
    let mut scene: Vec<Rc<dyn Hittable>> = Vec::new();
    let checker = Rc::new(CheckerTexture::new(
        Box::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
        Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
    ));
    scene.push(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, -1.0),
        1000.0,
        Rc::new(Lambertian::new(checker)),
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
                    scene.push(Rc::new(MovingSphere::new(
                        center,
                        center + Vec3::new(0.0, 0.5 * rand_float(), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
                            rand_float() * rand_float(),
                            rand_float() * rand_float(),
                            rand_float() * rand_float(),
                        ))))),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    scene.push(Rc::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Metal::new(
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
                    scene.push(Rc::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }
    scene.push(Rc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    scene.push(Rc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
            0.4, 0.2, 0.1,
        ))))),
    )));
    scene.push(Rc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));
    scene
}

pub fn two_spheres_scene() -> Vec<Rc<dyn Hittable>> {
    let mut scene: Vec<Rc<dyn Hittable>> = Vec::new();
    let checker = Rc::new(CheckerTexture::new(
        Box::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
        Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
    ));
    let checker2 = Rc::new(CheckerTexture::new(
        Box::new(ConstantTexture::new(Vec3::new(0.1, 0.2, 0.3))),
        Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
    ));
    scene.push(Rc::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Rc::new(Lambertian::new(checker)),
    )));
    scene.push(Rc::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Rc::new(Lambertian::new(checker2)),
    )));
    scene
}

pub fn two_perlin_spheres_scene() -> Vec<Rc<dyn Hittable>> {
    let mut scene: Vec<Rc<dyn Hittable>> = Vec::new();
    let perlin_texture = Rc::new(NoiseTexture::new(4.0, Perlin::new()));
    scene.push(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(perlin_texture.clone())),
    )));
    scene.push(Rc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new(perlin_texture)),
    )));
    scene
}

pub fn earth_scene() -> Vec<Rc<dyn Hittable>> {
    let mut scene: Vec<Rc<dyn Hittable>> = Vec::new();
    let perlin_texture = Rc::new(NoiseTexture::new(4.0, Perlin::new()));
    scene.push(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(perlin_texture)),
    )));

    let img = image::open("texture/earthmap.jpg").unwrap();
    let (nx, ny) = img.dimensions();
    let data = img.raw_pixels();
    let image_texture = Rc::new(ImageTexture::new(data, nx as i32, ny as i32));
    scene.push(Rc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new(image_texture)),
    )));
    scene
}

pub fn simple_light() -> Vec<Rc<dyn Hittable>> {
    let perlin_texture = Rc::new(NoiseTexture::new(4.0, Perlin::new()));
    let mut scene: Vec<Rc<dyn Hittable>> = Vec::new();
    scene.push(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(perlin_texture.clone())),
    )));
    scene.push(Rc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new(perlin_texture)),
    )));

    let constant_texture = Rc::new(ConstantTexture::new(Vec3::new(4.0, 4.0, 4.0)));
    scene.push(Rc::new(Sphere::new(
        Vec3::new(0.0, 7.0, 0.0),
        2.0,
        Rc::new(DiffuseLight::new(constant_texture.clone())),
    )));
    scene.push(Rc::new(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        Rc::new(DiffuseLight::new(constant_texture.clone())),
    )));
    scene
}

pub fn cornell_box() -> Vec<Rc<dyn Hittable>> {
    let mut scene: Vec<Rc<dyn Hittable>> = Vec::new();

    let red = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
        0.65, 0.05, 0.05,
    )))));
    let white = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
        0.73, 0.73, 0.73,
    )))));
    let green = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
        0.12, 0.45, 0.15,
    )))));
    let light = Rc::new(DiffuseLight::new(Rc::new(ConstantTexture::new(Vec3::new(
        15.0, 15.0, 15.0,
    )))));

    scene.push(Rc::new(FlipNormals::new(Rc::new(YZRect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, green,
    )))));
    scene.push(Rc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    scene.push(Rc::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    scene.push(Rc::new(FlipNormals::new(Rc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )))));
    scene.push(Rc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    scene.push(Rc::new(FlipNormals::new(Rc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )))));

    let small_box = Rc::new(BoxShape::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let tall_box = Rc::new(BoxShape::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    scene.push(Rc::new(Translate::new(
        Rc::new(RotateY::new(small_box, -18.0)),
        Vec3::new(130.0, 0.0, 65.0),
    )));
    scene.push(Rc::new(Translate::new(
        Rc::new(RotateY::new(tall_box, 15.0)),
        Vec3::new(265.0, 0.0, 295.0),
    )));

    scene
}

pub fn cornell_smoke_scene() -> Vec<Rc<dyn Hittable>> {
    let mut scene: Vec<Rc<dyn Hittable>> = Vec::new();
    let red = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
        0.65, 0.05, 0.05,
    )))));
    let white = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
        0.73, 0.73, 0.73,
    )))));
    let green = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
        0.12, 0.45, 0.15,
    )))));
    let light = Rc::new(DiffuseLight::new(Rc::new(ConstantTexture::new(Vec3::new(
        7.0, 7.0, 7.0,
    )))));

    scene.push(Rc::new(FlipNormals::new(Rc::new(YZRect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, green,
    )))));
    scene.push(Rc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    scene.push(Rc::new(XZRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    scene.push(Rc::new(FlipNormals::new(Rc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )))));
    scene.push(Rc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    scene.push(Rc::new(FlipNormals::new(Rc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )))));
    let tall_box = Rc::new(BoxShape::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let tall_box = Rc::new(RotateY::new(tall_box, 15.0));
    let tall_box = Rc::new(Translate::new(tall_box, Vec3::new(265.0, 0.0, 295.0)));
    let small_box = Rc::new(BoxShape::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let small_box = Rc::new(RotateY::new(small_box, -18.0));
    let small_box = Rc::new(Translate::new(small_box, Vec3::new(130.0, 0.0, 65.0)));
    scene.push(Rc::new(ConstantMedium::new(tall_box, 0.01, Rc::new(ConstantTexture::new(Vec3::new(0.0, 0.0, 0.0))))));
    scene.push(Rc::new(ConstantMedium::new(small_box, 0.01, Rc::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))))));
    scene
}
