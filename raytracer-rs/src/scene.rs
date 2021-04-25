use crate::shapes::*;
//use crate::vec3::Vec3;
use crate::ray::Ray;
//use crate::materials::Material;
use crate::hit_record::HitRecord;

use std::rc::Rc;

pub struct Scene {
    shapes: Vec<Rc<dyn Hittable>>
}

impl Scene {
    pub fn new() -> Scene {
        Scene{ shapes: Vec::new() }
    }

    pub fn add_shape<S: 'static + Hittable>(&mut self, shape: &S) {
        self.shapes.push(Rc::new(*shape));
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