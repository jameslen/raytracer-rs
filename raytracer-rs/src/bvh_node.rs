use crate::ray::Ray;
use crate::aabb::AABB;
use crate::shapes::Hittable;
use crate::scene::Scene;
use crate::hit_record::HitRecord;

use glam::*;

use rand::prelude::*;
use std::sync::Arc;
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
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bounding_box: AABB
}

fn axis_max(first: (f32, usize), second: (f32, usize)) -> (f32, usize) {
    if first.0 >= second.0 {
        return first;
    } else {
        return second;
    }
}

impl BVHNode {
    pub fn new() -> BVHNode {
        BVHNode {
            left: Arc::new(EmptyHittable{}),
            right: Arc::new(EmptyHittable{}),
            bounding_box: AABB::new()
        }
    }
    pub fn from_scene(scene: &Scene, t0: f32, t1: f32) -> BVHNode {
        return BVHNode::from_vector(scene.shapes[0..].to_vec(), t0, t1);
    }

    pub fn from_vector(objects: Vec<Arc<dyn Hittable>>, t0: f32, t1: f32) -> BVHNode {
        let mut objects = objects.clone();

        // let (min, max)  = objects.iter().fold((Vec3A::ONE * f32::INFINITY, Vec3A::ONE * -f32::INFINITY), |acc, object| {
        //     let obj_aabb = object.bounding_box(0.0,0.0).unwrap();

        //     (acc.0.min(obj_aabb.min), acc.1.max(obj_aabb.max))
        // });

        // let range = max - min;

        // let (extent, axis) = axis_max(axis_max((range.x, 0), (range.y, 1)), (range.z, 2));
        let mut rng = rand::thread_rng();

        let axis = rng.gen_range(0..3);

        let mut root = BVHNode::new();

        let span = objects.len();

        if span == 1 {
            root.left = objects[0].clone();
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
            
            objects.sort_by(|a,b| BVHNode::box_compare(a, b, axis));

            let mid = span / 2;
            let left = objects[..mid].to_vec();
            let right = objects[mid..].to_vec();

            if left.len() == 1 {
                root.left = left[0].clone();
            } else {
                root.left = Arc::new(BVHNode::from_vector(left, t0, t1));
            }
            if right.len() == 1 {
                root.right = right[0].clone()
            } else {
                root.right = Arc::new(BVHNode::from_vector(objects[mid..].to_vec(), t0, t1));
            }
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

        println!("Count {}, Box min: {:?}, max: {:?}", span, root.bounding_box.min, root.bounding_box.max);

        return root;
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> std::cmp::Ordering {
        
        let mut box_a = AABB::new();
        if let Some(aabb) = a.bounding_box(0.0, 0.0) {
            box_a = aabb;
        }

        let mut box_b = AABB::new();
        if let Some(aabb) = b.bounding_box(0.0, 0.0) {
            box_b = aabb;
        }

        if (box_a.min[axis] - box_b.min[axis]) < f32::EPSILON {
            return Ordering::Less;
        } else if (box_a.min[axis] - box_b.min[axis]) > f32::EPSILON {
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

        let mut t_max0 = t_max;
        let mut left_result = self.left.intersect(ray, t_min, t_max0);

        if let Option::Some(left) = left_result {
            t_max0 = left.t;
            left_result = Option::Some(left);
        }

        let right_result = self.right.intersect(ray, t_min, t_max0);

        match right_result {
            Option::Some(_) => return right_result,
            Option::None => return left_result
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        return Some(self.bounding_box);
    }
}

unsafe impl Send for BVHNode {}
unsafe impl Sync for BVHNode {}