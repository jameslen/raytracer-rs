use crate::ray::Ray;
use crate::aabb::AABB;
use crate::shapes::Hittable;
use crate::scene::Scene;
use crate::hit_record::HitRecord;

use rand::prelude::*;
use std::rc::Rc;
use std::cmp::Ordering;

struct EmptyHittable {

}

impl Hittable for EmptyHittable {
    fn intersect(&self, _ray: &Ray, _t_min: f32, _t_max: f32) -> Option<HitRecord> {
        return Option::None;
    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        return None;
    }
}

pub struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bounding_box: AABB
}

impl BVHNode {
    pub fn new() -> BVHNode {
        BVHNode {
            left: Rc::new(EmptyHittable{}),
            right: Rc::new(EmptyHittable{}),
            bounding_box: AABB::new()
        }
    }
    pub fn from_scene(scene: &Scene, t0: f32, t1: f32) -> BVHNode {
        return BVHNode::from_vector(scene.shapes[0..].to_vec(), t0, t1);
    }

    pub fn from_vector(objects: Vec<Rc<dyn Hittable>>, t0: f32, t1: f32) -> BVHNode {
        let mut objects = objects.clone();

        let mut root = BVHNode::new();

        let mut rng = rand::thread_rng();

        let axis = rng.gen_range(0..2);

        let span = objects.len();

        if span == 1 {
            root.left = objects[0].clone();
            root.right = root.left.clone();
        } else if span == 2 {
            let result = BVHNode::box_compare(&objects[0], &objects[1], axis);

            match result {
                Ordering::Less => {
                    root.left = objects[0].clone();
                    root.right = objects[1].clone();
                },
                _ => {
                    root.right = objects[0].clone();
                    root.left = objects[1].clone();
                }
            }
        } else {            
            
            objects.sort_unstable_by(|a,b| BVHNode::box_compare(a, b, axis));

            let mid = span / 2;
            root.left = Rc::new(BVHNode::from_vector(objects[..mid].to_vec(), t0, t1));
            root.right = Rc::new(BVHNode::from_vector(objects[mid..].to_vec(), t0, t1));
        }

        let mut left_aabb = AABB::new();
        let mut right_aabb = AABB::new();

        if let Some(aabb) = root.left.bounding_box(t0, t1) {
            left_aabb = aabb;
        }
        if let Some(aabb) = root.right.bounding_box(t0, t1) {
            right_aabb = aabb;
        }

        root.bounding_box = AABB::surrounding_box(&left_aabb, &right_aabb);

        return root;
    }

    fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: usize) -> std::cmp::Ordering {
        
        let mut box_a = AABB::new();
        if let Some(aabb) = a.bounding_box(0.0, 0.0) {
            box_a = aabb;
        }

        let mut box_b = AABB::new();
        if let Some(aabb) = b.bounding_box(0.0, 0.0) {
            box_b = aabb;
        }

        if box_a.min[axis] < box_b.min[axis] {
            return Ordering::Less;
        } else if box_a.min[axis] > box_b.min[axis] {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    }
}

impl Hittable for BVHNode {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bounding_box.hit(ray, t_min, t_max) == false {
            return Option::None;
        }

        let mut t_max = t_max;
        let mut left_result = self.left.intersect(ray, t_min, t_max);

        if let Option::Some(left) = left_result {
            t_max = left.t;
            left_result = Option::Some(left);
        }

        let right_result = self.right.intersect(ray, t_min, t_max);

        match right_result {
            Option::Some(_) => return right_result,
            Option::None => return left_result
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        return Some(self.bounding_box);
    }
}