use crate::shapes::Hittable;
use crate::ray::Ray;
use crate::aabb::AABB;
use crate::hit_record::HitRecord;

use std::rc::Rc;

pub struct Scene {
    pub shapes: Vec<Rc<dyn Hittable>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene{ shapes: Vec::new() }
    }

    pub fn add_shape<S: 'static + Hittable>(&mut self, shape: S) {
        self.shapes.push(Rc::new(shape));
    }

    pub fn clear(&mut self) {
        self.shapes.clear();
    }
}

impl Hittable for Scene {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;

        let mut scene_result = Option::None;
        
        for shape in self.shapes.iter() {
            let result = shape.intersect(ray, t_min, closest_so_far);
            if let Option::Some(hit_record) = result {
                closest_so_far = hit_record.t;
                scene_result = Option::Some(hit_record);
            }
        }

        return scene_result;
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.shapes.is_empty() {
            return None;
        }
        let mut first_box = true;
        let mut result = AABB::new();

        for shape in self.shapes.iter() {
            let temp: AABB;

            if let Some(aabb) = shape.bounding_box(t0, t1) {
                temp = aabb;
            } else {
                return None;
            }

            if first_box {
                result = temp;
                first_box = false;
            } else {
                result = AABB::surrounding_box(&temp, &result);
            }
        }

        return Some(result);
    }
}