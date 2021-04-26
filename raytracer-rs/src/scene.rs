use crate::shapes::Hittable;
use crate::vec3::Vec3;
use crate::ray::Ray;
//use crate::materials::Material;
use crate::hit_record::HitRecord;
use crate::aabb::AABB;

use std::rc::Rc;

pub struct Scene {
    pub shapes: Vec<Rc<dyn Hittable>>
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

    fn bounding_box(&self, t0: f32, t1: f32, aabb: &mut AABB) -> bool {
        if self.shapes.is_empty() {
            return false;
        }
        let mut first_box = true;

        for shape in self.shapes.iter() {
            let mut temp = AABB {
                min: Vec3{
                    x: -f32::INFINITY,
                    y: -f32::INFINITY,
                    z: -f32::INFINITY
                },
                max: Vec3{
                    x: f32::INFINITY,
                    y: f32::INFINITY,
                    z: f32::INFINITY
                }
            };

            if shape.bounding_box(t0, t1, &mut temp) == false {
                return false;
            }

            if first_box {
                *aabb = temp;
                first_box = false;
            } else {
                *aabb = AABB::surrounding_box(&temp, aabb);
            }
        }

        return true;

    }
}