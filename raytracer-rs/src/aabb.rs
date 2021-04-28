extern crate glam;

use glam::*;

use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct AABB {
    pub min: Vec3A,
    pub max: Vec3A
}

impl AABB {
    pub fn new() -> AABB {
        AABB {
            min: Vec3A::new(f32::INFINITY,f32::INFINITY,f32::INFINITY),
            max:  Vec3A::new(-f32::INFINITY,-f32::INFINITY,-f32::INFINITY),
        }
    }

    pub fn hit(&self, ray: &Ray, _t_min: f32, _t_max: f32) -> bool {
        let recip = ray.direction.recip();
        let min = (self.min - ray.origin) * recip;
        let max = (self.max - ray.origin) * recip;

        let t_min = f32::max(f32::max(f32::min(min.x, max.x), f32::min(min.y, max.y)), f32::min(min.z, max.z));
        let t_max = f32::min(f32::min(f32::max(min.x, max.x), f32::max(min.y, max.y)), f32::max(min.z, max.z));

        if t_max < 0.0 || t_min > t_max {
            return false;
        }
        return true;
    }

    pub fn surrounding_box(box1: &AABB, box2: &AABB) -> AABB {
        let small = box1.min.min(box2.min);

        let big = box1.max.max(box2.max);

        AABB {
            min: small,
            max: big
        }
    }
}