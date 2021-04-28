#![allow(dead_code)]

extern crate rand; 
extern crate image;

mod vec3_helpers;
mod ray;
mod shapes;
mod scene;
mod camera;
mod materials;
mod hit_record;
mod aabb;
mod bvh_node;
mod texture;
mod perlin;

use std::time::Instant;
use std::sync::Arc;
use std::slice;

use image::*;
use glam::*;
use rayon::prelude::*;

use ray::Ray;
use scene::Scene;
use shapes::*;
use camera::Camera;
use rand::prelude::*;
use materials::*;
use bvh_node::BVHNode;
use texture::*;


// fn write_color(color: &Vec3A, samples_per_pixel: f32) -> Rgb {
//     let scale = 1.0 / samples_per_pixel;
//     let r = f32::sqrt(color.x * scale);
//     let g = f32::sqrt(color.y * scale);
//     let b = f32::sqrt(color.z * scale);

//      (256.0 * f32::clamp(r,0.0, 0.999)) as u8, (256.0 * f32::clamp(g, 0.0, 0.999)) as u8, (256.0 * f32::clamp(b, 0.0, 0.999)) as u8);
// }

fn ray_color<T: Hittable + Send>(ray: &Ray, background: Vec3A, world: &T, depth: i32) -> Vec3A {
    if depth <= 0 {
        return Vec3A::ZERO;
    }

    let world_result = world.intersect(ray, 0.005, f32::INFINITY);
    if let Option::Some(record) = world_result {
        let mut scattered = Ray{ origin: Vec3A::ZERO, direction: Vec3A::ZERO, time: ray.time };
        let mut attentuation = Vec3A::ONE;
        let emitted = record.material.emitted(record.tex_coords, record.point);

        if record.material.scatter(ray, &record, &mut attentuation, &mut scattered) {
            return emitted + attentuation * ray_color(&scattered, background, world, depth - 1);
        } else {
            return emitted;
        }
    } else {
        return background;
    }
}

fn simple_ray_color(ray: &Ray, background: Vec3A, world: &dyn Hittable, _depth: i32) -> Vec3A {

    let world_result = world.intersect(ray, 0.005, f32::INFINITY);
    if let Option::Some(record) = world_result {
        let mut scattered = Ray{ origin: Vec3A::ZERO, direction: Vec3A::ZERO, time: ray.time };
        let mut attentuation = Vec3A::ONE;
        let emitted = record.material.emitted(record.tex_coords, record.point);

        if record.material.scatter(ray, &record, &mut attentuation, &mut scattered) {
            return attentuation;
        } else {
            return emitted;
        }
    } else {
        return background;
    }
}

fn degree_to_rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

pub fn generate_random_world() -> Scene {
    let mut s = Scene::new();

    // Ground
    let material = LambertianMat::from_texture(CheckeredTexture::from_color(
        Vec3A::new(0.2, 0.3, 0.1),
        Vec3A::new(0.9, 0.9, 0.9),
    ));
    s.add_shape(Sphere::new(Vec3A::new(0.0, -1000.0, 0.0), 1000.0, material));
    
    let mut rng = rand::thread_rng();

    let point = Vec3A::new(4.0, 0.2, 0.0);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();

            let center = Vec3A::new(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 * 0.9 * rng.gen::<f32>());

            if (center - point).length() > 0.9 {
                if choose_mat < 0.8 {
                    let center2 = center + Vec3A::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    s.add_shape(MovingSphere::new(center, center2, 0.2, 0.0, 1.0, LambertianMat::from_color(vec3_helpers::random() * vec3_helpers::random())));
                } else if choose_mat < 0.95 {
                    s.add_shape(Sphere::new(center, 0.2, MetalMat::new(vec3_helpers::random_range(0.5,1.0), rng.gen_range(0.5..1.0))));
                } else {
                    s.add_shape(Sphere::new(center, 0.2, DielectricMat::new(1.5)));
                }
            }
        }
    }

    s.add_shape(Sphere::new(Vec3A::new( 0.0, 1.0, 0.0), 1.0, DielectricMat::new(1.5)));
    s.add_shape(Sphere::new(Vec3A::new(-4.0, 1.0, 0.0), 1.0, LambertianMat::from_color(Vec3A::new(0.4, 0.2, 0.1))));
    s.add_shape(Sphere::new(Vec3A::new( 4.0, 1.0, 0.0), 1.0, MetalMat::new(Vec3A::new(0.7, 0.6, 0.5), 0.0)));

    return s;
}

fn two_spheres() -> Scene {
    let mut s = Scene::new();

    let checkered = Arc::new(CheckeredTexture::from_color(
        Vec3A::new(0.2, 0.3, 0.1),
        Vec3A::new(0.9, 0.9, 0.9),
    ));

    s.add_shape(Sphere::new(Vec3A::new(0.0,  10.0, 0.0), 10.0, LambertianMat::from_shared_texture(checkered.clone())));
    s.add_shape(Sphere::new(Vec3A::new(0.0, -10.0, 0.0), 10.0, LambertianMat::from_shared_texture(checkered.clone())));

    return s;
}

fn two_perlin_spheres() -> Scene {
    let mut s = Scene::new();

    let noise_texture = Arc::new(NoiseTexture::new(4.0));

    s.add_shape(Sphere::new(Vec3A::new(0.0, -1000.0, 0.0), 1000.0, LambertianMat::from_shared_texture(noise_texture.clone())));
    s.add_shape(Sphere::new(Vec3A::new(0.0, 2.0, 0.0), 2.0, LambertianMat::from_shared_texture(noise_texture.clone())));

    return s;
}

fn earth() -> Scene {
    let mut s = Scene::new();

    let texture = Arc::new(ImageTexture::new(String::from("earthmap.jpg")));
    let surface = LambertianMat::from_shared_texture(texture);
    
    s.add_shape(Sphere::new(Vec3A::ZERO, 2.0, surface));

    return s;
}

fn simple_light() -> Scene {
    let mut s = Scene::new();

    let noise_texture = Arc::new(NoiseTexture::new(4.0));

    s.add_shape(Sphere::new(Vec3A::new(0.0, -1000.0, 0.0), 1000.0, LambertianMat::from_shared_texture(noise_texture.clone())));
    s.add_shape(Sphere::new(Vec3A::new(0.0, 2.0, 0.0), 2.0, LambertianMat::from_shared_texture(noise_texture.clone())));

    s.add_shape(XYRect::new(Vec2::new(3.0, 1.0), Vec2::new(5.0, 3.0), -2.0, DiffuseLight::from_color(Vec3A::new(4.0, 4.0, 4.0))));
    s.add_shape(Sphere::new(Vec3A::new(0.0, 7.0, 0.0), 2.0, DiffuseLight::from_color(Vec3A::new(4.0, 4.0, 4.0))));

    return s;
}

fn cornell_box() -> Scene {
    let mut s = Scene::new();

    let white = Vec3A::new(0.73, 0.73, 0.73);
    let green = Vec3A::new(0.12, 0.45, 0.15);
    let red   = Vec3A::new(0.65, 0.05, 0.05);
    let light = Vec3A::new(15.0, 15.0, 15.0);

    // //s.add_shape(YZRect::new(Vec2::new(0.0, 0.0), Vec2::new(555.0, 555.0), 555.0, LambertianMat::from_color(green)));
    // //s.add_shape(YZRect::new(Vec2::new(0.0, 0.0), Vec2::new(555.0, 555.0), 0.0, LambertianMat::from_color(red)));
    // s.add_shape(XZRect::new(Vec2::new(213.0, 227.0), Vec2::new(343.0, 332.0), 554.0, DiffuseLight::from_color(light)));
    // //s.add_shape(XZRect::new(Vec2::new(0.0, 0.0), Vec2::new(555.0, 555.0), 555.0, LambertianMat::from_color(white)));
    // //s.add_shape(XZRect::new(Vec2::new(0.0, 0.0), Vec2::new(555.0, 555.0), 0.0, LambertianMat::from_color(white)));
    // //s.add_shape(XYRect::new(Vec2::new(0.0, 0.0), Vec2::new(555.0, 555.0), 555.0, LambertianMat::from_color(white)));

    // let b2 = Box::new(165.0, 165.0, 165.0, LambertianMat::from_color(white));
    // let rotation = Mat4::from_rotation_y(degree_to_rad(-18.0));
    // let translation = Mat4::from_translation(Vec3::new(130.0, 0.0, 65.0));
    // let final_transform = translation * rotation;
    // s.add_shape(TransformedObject::new(b2, final_transform));

    // let b1 = Box::new(165.0, 330.0, 165.0, LambertianMat::from_color(white));
    // let rotation = Mat4::from_rotation_y(degree_to_rad(15.0));
    // let translation = Mat4::from_translation(Vec3::new(265.0, 0.0, 305.0));
    // let final_transform = translation * rotation;
    //s.add_shape(TransformedObject::new(b1, final_transform));

    // Light
    let min = Vec3A::new(213.0, 553.999, 227.0);
    let max = Vec3A::new(343.0, 554.001, 332.0);
    let range = max - min;
    let b1 = Box::new(range.x, range.y, range.z, LambertianMat::from_color(white));
    let translation = Mat4::from_translation(Vec3::new(min.x, min.y, min.z));
    s.add_shape(TransformedObject::new(b1, translation));

    // Small Box
    let min = Vec3A::new(130.0, 0.0, 65.0);
    let max = Vec3A::new(235.93652, 165.0, 272.91214);
    let range = max - min;
    let b1 = Box::new(range.x, range.y, range.z, LambertianMat::from_color(red));
    let translation = Mat4::from_translation(Vec3::new(min.x, min.y, min.z));
    s.add_shape(TransformedObject::new(b1, translation));

    // Large Box
    let min = Vec3A::new(265.0, 0.0, 305.0);
    let max = Vec3A::new(467.0829, 330.0, 421.6726);
    let range = max - min;
    let b1 = Box::new(range.x, range.y, range.z, LambertianMat::from_color(green));
    let translation = Mat4::from_translation(Vec3::new(min.x, min.y, min.z));
    s.add_shape(TransformedObject::new(b1, translation));

    return s;
}

fn float_to_u8_color(f: f32) -> u8 {
    (256.0 * f32::clamp(f, 0.0, 0.999)) as u8
}

#[allow(dead_code)]
enum ImageQuality {
    Low,
    High,
    Cornell
}

#[allow(dead_code)]
enum SceneType {
    Random,
    TwoSpheres,
    PerlinSpheres,
    Earth,
    SimpleLight,
    CornellBox
}

fn main() {

    // TODO: get dimensions from CLI
    // Image
    let aspect_ratio: f32;
    let image_width: u32;
    let samples_per_pixel: u32;
    let max_depth: i32;

    let world: Scene;
    let fov: f32;
    let aperture: f32;
    let focus_distance: f32;
    let origin: Vec3A;
    let target: Vec3A;

    let background: Vec3A;

    let quality = ImageQuality::Cornell;
    let scene = SceneType::CornellBox;

    match quality {
        ImageQuality::Low => {
            aspect_ratio = 16.0 / 9.0;
            image_width = 400;
            samples_per_pixel = 100;
            max_depth = 50;
        },
        ImageQuality::High => {
            aspect_ratio = 3.0 / 2.0;
            image_width = 1600;
            samples_per_pixel = 500;
            max_depth = 50;
        }
        ImageQuality::Cornell => {
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            max_depth = 50;
        }
    }

    let image_height = (image_width as f32 / aspect_ratio) as u32;

    match scene {
        SceneType::Random => {
            world = generate_random_world();
            origin = Vec3A::new(13.0, 2.0, 3.0);
            target = Vec3A::new(0.0, 0.0, 0.0);
            fov = degree_to_rad(20.0);
            aperture = 0.1;
            background = Vec3A::new(0.70, 0.80, 1.00);
        },
        SceneType::TwoSpheres => {
            world = two_spheres();
            origin = Vec3A::new(13.0, 2.0, 3.0);
            target = Vec3A::new(0.0, 0.0, 0.0);
            fov = degree_to_rad(20.0);
            aperture = 0.0;
            background = Vec3A::new(0.7, 0.8, 1.0);
        },
        SceneType::PerlinSpheres => {
            world = two_perlin_spheres();
            origin = Vec3A::new(13.0, 2.0, 3.0);
            target = Vec3A::new(0.0, 0.0, 0.0);
            fov = degree_to_rad(20.0);
            aperture = 0.0;
            background = Vec3A::new(0.70, 0.80, 1.00);
        },
        SceneType::Earth => {
            world = earth();
            origin = Vec3A::new(13.0, 2.0, 3.0);
            target = Vec3A::new(0.0, 0.0, 0.0);
            fov = degree_to_rad(20.0);
            aperture = 0.0;
            background = Vec3A::new(0.70, 0.80, 1.00);
        },
        SceneType::SimpleLight => {
            world = simple_light();
            origin = Vec3A::new(26.0, 3.0, 6.0);
            target = Vec3A::new(0.0, 2.0, 0.0);
            fov = degree_to_rad(20.0);
            aperture = 0.0;
            background = Vec3A::ZERO;
        },
        SceneType::CornellBox => {
            world = cornell_box();
            origin = Vec3A::new(278.0, 278.0, -800.0);
            target = Vec3A::new(278.0, 278.0, 0.0);
            fov = degree_to_rad(40.0);
            aperture = 0.0;
            //background = Vec3A::ZERO;
            background = Vec3A::new(0.7, 0.8, 1.0);
        }
    }

    

    let vup = Vec3A::Y;
    focus_distance = 10.0;
    let camera = Camera::new(origin, target, vup, fov, aspect_ratio, aperture, focus_distance, 0.0, 1.0); 

    println!("");
    for obj in world.shapes.iter() {
        let aabb = obj.bounding_box(0.0, 1.0).unwrap();
        println!("post: min: {:?}, max: {:?}", aabb.min, aabb.max);
    }

    let bvh = BVHNode::from_scene(&world, 0.0, 1.0);

    let output: RgbImage = ImageBuffer::new(image_width, image_height);

    let inv_samples = 1.0 / samples_per_pixel as f32;

    let now = Instant::now();
    let x: Vec<(u32, u32, (u8,u8,u8))> = output
        .enumerate_pixels()
        .map(|(i, j, pixel)| {
            let channels = pixel.channels4();
            (i, j, (channels.0, channels.1, channels.2))
        })
        .collect();
    let par: Vec<Rgb<u8>> = x.par_iter().map(|(i, j, pixel)| {
        let mut rng = rand::thread_rng();
        let mut color = Vec3A::ZERO;
        for _sample in 0..samples_per_pixel {
            let u = (*i as f32 + rng.gen::<f32>()) / (image_width - 1) as f32;
            let v = ((image_height - 1 - *j) as f32 + rng.gen::<f32>()) / (image_height - 1) as f32;

            let r = camera.get_ray(u, v);

            color += ray_color(&r, background, &bvh, max_depth);
        }

        color *= inv_samples;
        
        image::Rgb::from_channels(float_to_u8_color(color.x), float_to_u8_color(color.y), float_to_u8_color(color.z), 0)
    }).collect();
    println!("Time elapsed: {}", now.elapsed().as_millis());

    let mut out_data = Vec::new();
    out_data.reserve(par.len() * 3);

    for pixel in par.iter() {
        out_data.push(pixel.0[0]);
        out_data.push(pixel.0[1]);
        out_data.push(pixel.0[2]);
    }

    let output: RgbImage = ImageBuffer::from_raw(image_width, image_height, out_data).unwrap();

    output.save("./test.png").unwrap();
}
