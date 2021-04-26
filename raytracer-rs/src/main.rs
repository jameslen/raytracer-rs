extern crate rand; 

mod vec3;
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

use std::fs::File;
use std::io::Write;
use std::time::Instant;
use std::rc::Rc;

use ray::Ray;
use vec3::Vec3;
use scene::Scene;
use shapes::*;
use camera::Camera;
use rand::prelude::*;
use materials::*;
use bvh_node::BVHNode;
use texture::*;


fn write_color(color: &Vec3, samples_per_pixel: f32) -> String {
    let scale = 1.0 / samples_per_pixel;
    let r = f32::sqrt(color.x * scale);
    let g = f32::sqrt(color.y * scale);
    let b = f32::sqrt(color.z * scale);

    return format!("{} {} {}\n", (256.0 * f32::clamp(r,0.0, 0.999)) as u8, (256.0 * f32::clamp(g, 0.0, 0.999)) as u8, (256.0 * f32::clamp(b, 0.0, 0.999)) as u8);
}

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3{ x: 0.0, y: 0.0, z: 0.0 };
    }

    let world_result = world.intersect(ray, 0.005, f32::INFINITY);
    if let Option::Some(record) = world_result {
        let mut scattered = Ray{ origin: Vec3{ x: 0.0, y: 0.0, z: 0.0 }, direction: Vec3{ x: 0.0, y: 0.0, z: 0.0 }, time: ray.time };
        let mut attentuation = Vec3{ x: 1.0, y: 1.0, z: 1.0 };

        if record.material.scatter(ray, &record, &mut attentuation, &mut scattered) {
            return attentuation * ray_color(&scattered, world, depth - 1);
        }
        return Vec3{x: 0.0, y: 0.0, z: 0.0 };
    }

    let normalized = ray.direction.normalize();
    let t = 0.5_f32 * (normalized.y + 1.0);

    &((1_f32 - t) * &Vec3{x: 1_f32, y: 1_f32, z: 1_f32}) + &(t * &Vec3{x: 0.5_f32, y: 0.7_f32, z: 1_f32})
}

fn degree_to_rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

pub fn generate_random_world() -> Scene {
    let mut s = Scene::new();

    // Ground
    let material = LambertianMat::from_texture(CheckeredTexture::from_color(
        Vec3{ x: 0.2, y: 0.3, z: 0.1 },
        Vec3{ x: 0.9, y: 0.9, z: 0.9 },
    ));
    s.add_shape(Sphere::new(Vec3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, material));
    
    let mut rng = rand::thread_rng();

    let point = Vec3 {
        x: 4.0,
        y: 0.2,
        z: 0.0
    };

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();

            let center = Vec3{
                x: a as f32 + 0.9 * rng.gen::<f32>(),
                y: 0.2,
                z: b as f32 * 0.9 * rng.gen::<f32>()
            };

            if (center - point).length() > 0.9 {
                if choose_mat < 0.8 {
                    let center2 = center + Vec3{ x: 0.0, y: rng.gen_range(0.0..0.5), z: 0.0 };
                    s.add_shape(MovingSphere::new(center, center2, 0.2, 0.0, 1.0, LambertianMat::from_color(Vec3::random() * Vec3::random())));
                } else if choose_mat < 0.95 {
                    s.add_shape(Sphere::new(center, 0.2, MetalMat::new(Vec3::random_range(0.5,1.0), rng.gen_range(0.5..1.0))));
                } else {
                    s.add_shape(Sphere::new(center, 0.2, DielectricMat::new(1.5)));
                }
            }
        }
    }

    s.add_shape(Sphere::new(Vec3{x: 0.0, y: 1.0, z: 0.0}, 1.0, DielectricMat::new(1.5)));
    s.add_shape(Sphere::new(Vec3{x:-4.0, y: 1.0, z: 0.0}, 1.0, LambertianMat::from_color(Vec3{x: 0.4, y: 0.2, z: 0.1})));
    s.add_shape(Sphere::new(Vec3{x: 4.0, y: 1.0, z: 0.0}, 1.0, MetalMat::new(Vec3{x:0.7, y: 0.6, z: 0.5}, 0.0)));

    return s;
}

fn two_spheres() -> Scene {
    let mut s = Scene::new();

    let checkered = Rc::new(CheckeredTexture::from_color(
        Vec3{ x: 0.2, y: 0.3, z: 0.1 },
        Vec3{ x: 0.9, y: 0.9, z: 0.9 },
    ));

    s.add_shape(Sphere::new(Vec3{x: 0.0, y:  10.0, z: 0.0}, 10.0, LambertianMat::from_shared_texture(checkered.clone())));
    s.add_shape(Sphere::new(Vec3{x: 0.0, y: -10.0, z: 0.0}, 10.0, LambertianMat::from_shared_texture(checkered.clone())));

    return s;
}

fn two_perlin_spheres() -> Scene {
    let mut s = Scene::new();

    let noise_texture = Rc::new(NoiseTexture::new(4.0));

    s.add_shape(Sphere::new(Vec3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, LambertianMat::from_shared_texture(noise_texture.clone())));
    s.add_shape(Sphere::new(Vec3{x: 0.0, y: 2.0, z: 0.0}, 2.0, LambertianMat::from_shared_texture(noise_texture.clone())));

    return s;
}

enum ImageQuality {
    Low,
    High
}

enum SceneType {
    Random,
    TwoSpheres,
    PerlinSpheres
}

fn main() {

    // TODO: get dimensions from CLI
    // Image
    let aspect_ratio: f32;
    let image_width: u32;
    let image_height: u32;
    let samples_per_pixel: u32;
    let max_depth: i32;

    let world: Scene;
    let fov: f32;
    let aperture: f32;
    let focus_distance: f32;
    let origin: Vec3;
    let target: Vec3;

    let quality = ImageQuality::Low;
    let scene = SceneType::PerlinSpheres;

    match quality {
        ImageQuality::Low => {
            aspect_ratio = 16.0 / 9.0;
            image_width = 400;
            image_height = (image_width as f32 / aspect_ratio) as u32;
            samples_per_pixel = 100;
            max_depth = 50;
        },
        ImageQuality::High => {
            aspect_ratio = 3.0 / 2.0;
            image_width = 1600;
            image_height = (image_width as f32 / aspect_ratio) as u32;
            samples_per_pixel = 500;
            max_depth = 50;
        }
    }

    match scene {
        SceneType::Random => {
            world = generate_random_world();
            origin = Vec3{x: 13.0, y: 2.0, z: 3.0};
            target = Vec3{x: 0.0, y: 0.0, z: 0.0};
            fov = degree_to_rad(20.0);
            aperture = 0.1;
        },
        SceneType::TwoSpheres => {
            world = two_spheres();
            origin = Vec3{x: 13.0, y: 2.0, z: 3.0};
            target = Vec3{x: 0.0, y: 0.0, z: 0.0};
            fov = degree_to_rad(20.0);
            aperture = 0.0;
        },
        SceneType::PerlinSpheres => {
            world = two_perlin_spheres();
            origin = Vec3{x: 13.0, y: 2.0, z: 3.0};
            target = Vec3{x: 0.0, y: 0.0, z: 0.0};
            fov = degree_to_rad(20.0);
            aperture = 0.0;
        }
    }
    

    let mut rng = rand::thread_rng();

    // TODO: get the file name from cli
    let mut file = File::create("./test.ppm").expect("Could not create file");

    let image_header = format!("P3\n{} {}\n255\n", image_width, image_height);

    file.write_all(image_header.as_bytes()).expect("Could not write image header");

    let vup = Vec3{x: 0.0, y: 1.0, z: 0.0};
    focus_distance = 10.0;
    let camera = Camera::new(origin, target, vup, fov, aspect_ratio, aperture, focus_distance, 0.0, 1.0); 
    let bvh = BVHNode::from_scene(&world, 0.0, 1.0);

    let now = Instant::now();
    for j in 0..image_height {
        for i in 0..image_width {
            let mut color = Vec3{ x: 0.0, y: 0.0, z: 0.0 };
            for _sample in 0..samples_per_pixel {
                let u = (i as f32 + rng.gen::<f32>()) / (image_width - 1) as f32;
                let v = ((image_height - 1 - j) as f32 + rng.gen::<f32>()) / (image_height - 1) as f32;

                let r = camera.get_ray(u, v);

                color += ray_color(&r, &bvh, max_depth);
            }
            let color_string : String = write_color(&color, samples_per_pixel as f32); 
            file.write_all(color_string.as_bytes()).expect("Couldn't write color");
        }
    }
    println!("Time elapsed: {}", now.elapsed().as_millis());
}
