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

use std::fs::File;
use std::io::Write;

use ray::Ray;
use vec3::Vec3;
use scene::Scene;
use shapes::*;
use camera::Camera;
use rand::prelude::*;
use materials::*;
use hit_record::*;


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
    let mut record = HitRecord::new();

    if world.intersect(ray, 0.005, f32::INFINITY, &mut record) == true {
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
    s.add_shape(Sphere::new(Vec3{x: 0.0, y: -1000.0, z: 0.0}, 1000.0, LambertianMat::new(Vec3{x: 0.5, y: 0.5, z: 0.5})));
    
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
                    s.add_shape(MovingSphere::new(center, center2, 0.2, 0.0, 1.0, LambertianMat::new(Vec3::random() * Vec3::random())));
                } else if choose_mat < 0.95 {
                    s.add_shape(Sphere::new(center, 0.2, MetalMat::new(Vec3::random_range(0.5,1.0), rng.gen_range(0.5..1.0))));
                } else {
                    s.add_shape(Sphere::new(center, 0.2, DielectricMat::new(1.5)));
                }
            }
        }
    }

    s.add_shape(Sphere::new(Vec3{x: 0.0, y: 1.0, z: 0.0}, 1.0, DielectricMat::new(1.5)));
    s.add_shape(Sphere::new(Vec3{x:-4.0, y: 1.0, z: 0.0}, 1.0, LambertianMat::new(Vec3{x: 0.4, y: 0.2, z: 0.1})));
    s.add_shape(Sphere::new(Vec3{x: 4.0, y: 1.0, z: 0.0}, 1.0, MetalMat::new(Vec3{x:0.7, y: 0.6, z: 0.5}, 0.0)));

    return s;
}

fn main() {
    // let mut world = Scene::new();
    // world.add_sphere(&Vec3{x: 0.0, y: -100.5, z: -1.0}, 100.0, LambertianMat::new(Vec3{x: 0.8, y: 0.8, z: 0.0}));
    // world.add_sphere(&Vec3{x: 0.0, y: 0.0, z: -1.0}, 0.5, LambertianMat::new(Vec3{x: 0.1, y: 0.2, z: 0.5}));
    // world.add_sphere(&Vec3{x:-1.0, y: 0.0, z: -1.0}, 0.5, DielectricMat::new(1.5));
    // world.add_sphere(&Vec3{x:-1.0, y: 0.0, z: -1.0},-0.45, DielectricMat::new(1.5));
    // world.add_sphere(&Vec3{x: 1.0, y: 0.0, z: -1.0}, 0.5, MetalMat::new(Vec3{x: 0.8, y: 0.6, z: 0.2}, 0.0));
    
    let world = generate_random_world();

    // TODO: get dimensions from CLI
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // Camera
    let origin = Vec3{x: 13.0, y: 2.0, z: 3.0};
    let target = Vec3{x: 0.0, y: 0.0, z: 0.0};
    let camera = Camera::new(origin, target, Vec3{x: 0.0, y: 1.0, z: 0.0}, degree_to_rad(20.0), aspect_ratio, 0.1, 10.0, 0.0, 1.0); 

    let mut rng = rand::thread_rng();

    // TODO: get the file name from cli
    let mut file = File::create("./test.ppm").expect("Could not create file");

    let image_header = format!("P3\n{} {}\n255\n", image_width, image_height);

    file.write_all(image_header.as_bytes()).expect("Could not write image header");

    for j in 0..image_height {
        for i in 0..image_width {
            let mut sample = 0;
            let mut color = Vec3{ x: 0.0, y: 0.0, z: 0.0 };
            while sample < samples_per_pixel {
                let u = (i as f32 + rng.gen::<f32>()) / (image_width - 1) as f32;
                let v = ((image_height - 1 - j) as f32 + rng.gen::<f32>()) / (image_height - 1) as f32;

                let r = camera.get_ray(u, v);

                color += ray_color(&r, &world, max_depth);
                sample += 1;
            }
            let color_string : String = write_color(&color, samples_per_pixel as f32); 
            file.write_all(color_string.as_bytes()).expect("Couldn't write color");
        }
    }
}
