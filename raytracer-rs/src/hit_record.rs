extern crate glam;

use glam::*;

use crate::ray::Ray;
use crate::materials::{Material, NoMaterial};

use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Vec3A,
    pub normal: Vec3A,
    pub t: f32,
    pub tex_coords: (f32, f32),
    pub material: Rc<dyn Material>,
    pub front_face: bool
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord{
            point: Vec3A::ZERO,
            normal: Vec3A::ZERO,
            t: f32::INFINITY,
            tex_coords: (0.0, 0.0),
            material: Rc::new(NoMaterial{}),
            front_face: false
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3A) {
        self.front_face = ray.direction.dot(*outward_normal) < 0.0;
        self.normal = {
            if self.front_face {
                *outward_normal
            }
            else {
                -*outward_normal
            }
        };
    }
}