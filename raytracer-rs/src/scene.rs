use crate::shapes::*;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::materials::Material;
use crate::hit_record::HitRecord;

use std::rc::Rc;

pub struct Scene {
    shapes: Vec<Sphere>
}

impl Scene {
    pub fn new() -> Scene {
        Scene{ shapes: Vec::new() }
    }

    pub fn add_sphere<T: 'static + Material>(&mut self, center: &Vec3, radius: f32, material: T) {
        self.shapes.push(Sphere{ center: *center, radius: radius, material: Rc::new(material)});
    }

    pub fn clear(&mut self) {
        self.shapes.clear();
    }
}

impl Hittable for Scene {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        
        for shape in self.shapes.iter() {
            if shape.intersect(ray, t_min, closest_so_far, record) == true {
                hit_anything = true;
                closest_so_far = record.t;
            }
        }

        return hit_anything;
    }
}