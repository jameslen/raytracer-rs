extern crate rand; 

mod vec3;
mod ray;
mod shapes;
mod scene;
mod camera;

use std::fs::File;
use std::io::Write;

use ray::Ray;
use vec3::Vec3;
use scene::Scene;
use shapes::*;
use camera::Camera;
use rand::prelude::*;


fn write_color(color: &Vec3, samples_per_pixel: f32) -> String {
    let scale = 1.0 / samples_per_pixel;
    let r = f32::sqrt(color.x * scale);
    let g = f32::sqrt(color.y * scale);
    let b = f32::sqrt(color.z * scale);

    return format!("{} {} {}\n", (256.0 * r.clamp(0.0, 0.999)) as u8, (256.0 * g.clamp(0.0, 0.999)) as u8, (256.0 * b.clamp(0.0, 0.999)) as u8);
}

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3{ x: 0.0, y: 0.0, z: 0.0 };
    }
    let mut record = HitRecord::new();

    if world.intersect(ray, 0.001, f32::INFINITY, &mut record) == true {
        let mut scattered = Ray{ origin: Vec3{ x: 0.0, y: 0.0, z: 0.0 }, direction: Vec3{ x: 0.0, y: 0.0, z: 0.0 } };
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

fn main() {
    let mut world = Scene::new();
    world.add_sphere(&Vec3{x: 0.0, y: -100.5, z: -1.0}, 100.0, LambertianMat::new(Vec3{x: 0.8, y: 0.8, z: 0.0}));
    world.add_sphere(&Vec3{x: 0.0, y: 0.0, z: -1.0}, 0.5, LambertianMat::new(Vec3{x: 0.1, y: 0.2, z: 0.5}));
    world.add_sphere(&Vec3{x:-1.0, y: 0.0, z: -1.0}, 0.5, DielectricMat::new(1.5));
    world.add_sphere(&Vec3{x: 1.0, y: 0.0, z: -1.0}, 0.5, MetalMat::new(Vec3{x: 0.8, y: 0.6, z: 0.2}, 0.0));
    

    // TODO: get dimensions from CLI
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // Camera
    let viewport_height = 2.0;
    let focal_length = 1.0;
    let camera = Camera::new(Vec3{x: 0.0, y: 0.0, z: 0.0}, viewport_height, aspect_ratio, focal_length); 

    let mut rng = rand::thread_rng();

    // TODO: get the file name from cli
    let mut file = File::create("./test.ppm").expect("Could not create file");

    let image_header = format!("P3\n{} {}\n255\n", image_width, image_height);

    file.write_all(image_header.as_bytes());

    let mut j = (image_height - 1) as i32;
    while j >= 0 {
        let mut i = 0;
        while i < image_width {
            let mut sample = 0;
            let mut color = Vec3{ x: 0.0, y: 0.0, z: 0.0 };
            while sample < samples_per_pixel {
                let u = (i as f32 + rng.gen::<f32>()) / (image_width - 1) as f32;
                let v = (j as f32 + rng.gen::<f32>()) / (image_height - 1) as f32;

                let r = camera.get_ray(u, v);

                color += ray_color(&r, &world, max_depth);
                sample += 1;
            }
            let color_string : String = write_color(&color, samples_per_pixel as f32); 
            file.write_all(color_string.as_bytes()).expect("Couldn't write color");

            i = i + 1;
        }
        j = j - 1;
    }
}
