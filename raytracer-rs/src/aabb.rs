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

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;
        for i in 0..2 {
            let inv_d = 1.0 / ray.direction[i];
            let mut t0 = (self.min[i] - ray.origin[i]) * inv_d;
            let mut t1 = (self.max[i] - ray.origin[i]) * inv_d;

            if inv_d < 0.0 {
                let temp = t0;
                t0 = t1;
                t1 = temp;
            }
            
            if t0 > t_min {
                t_min = t0;
            }
            if t1 < t_max {
                t_max = t1;
            }

            if t_max <= t_min {
                return false;
            }
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