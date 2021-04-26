use crate::ray::Ray;
use crate::aabb::AABB;
use crate::shapes::Hittable;
use crate::scene::Scene;
use crate::hit_record::HitRecord;

use rand::prelude::*;
use std::rc::Rc;

struct EmptyHittable {

}

impl Hittable for EmptyHittable {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        return false;
    }
    fn bounding_box(&self, t0: f32, t1: f32, aabb: &mut AABB) -> bool {
        return false;
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
        let mut root = BVHNode::new();

        let mut rng = rand::thread_rng();

        let axis = rng.gen_range(0..2);

        let comparator = |a, b| BVHNode::box_compare(a, b, axis);

        let span = objects.len();

        if span == 1 {
            root.left = objects[0].clone();
            root.right = root.left.clone();
        } else {            objects.sort_unstable_by(comparator);

            let mid = span / 2;
            root.left = Rc::new(BVHNode::from_vector(objects[..mid].to_vec(), t0, t1));
            root.right = Rc::new(BVHNode::from_vector(objects[mid..].to_vec(), t0, t1));
        }

        let mut left_aabb = AABB::new();
        let mut right_aabb = AABB::new();

        root.left.bounding_box(t0, t1, &mut left_aabb);
        root.right.bounding_box(t0, t1, &mut right_aabb);

        root.bounding_box = AABB::surrounding_box(&left_aabb, &right_aabb);

        return root;
    }

    fn box_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>, axis: usize) -> bool {
        return false;
    }
}

impl Hittable for BVHNode {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        if self.bounding_box.hit(ray, t_min, t_max) == false {
            return false;
        }

        let hit_left = self.left.intersect(ray, t_min, t_max, record);

        let mut hit_right: bool;
         {
            let mut t_max = t_max; 
            if hit_left {
                t_max = record.t;
            }

            hit_right = self.right.intersect(ray, t_min, t_max, record);
        }

        return hit_left || hit_right;
    }

    fn bounding_box(&self, t0: f32, t1: f32, aabb: &mut AABB) -> bool {
        *aabb = self.bounding_box;
        return true;
    }
}