use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::materials::{Material, NoMaterial};

use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub tex_coords: (f32, f32),
    pub material: Rc<dyn Material>,
    pub front_face: bool
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord{
            point: Vec3{ x: 0.0, y: 0.0, z: 0.0 },
            normal: Vec3{ x: 0.0, y: 0.0, z: 0.0 },
            t: f32::INFINITY,
            tex_coords: (0.0, 0.0),
            material: Rc::new(NoMaterial{}),
            front_face: false
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = {
            if self.front_face {
                *outward_normal
            }
            else {
                -outward_normal
            }
        };
    }
}