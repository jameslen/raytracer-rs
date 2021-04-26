use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct AABB {
    pub min: Vec3,
    pub max: Vec3
}

impl AABB {
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
        let small = Vec3 {
            x: f32::min(box1.min.x, box2.min.x),
            y: f32::min(box1.min.y, box2.min.y),
            z: f32::min(box1.min.z, box2.min.z),
        };

        let big = Vec3 {
            x: f32::max(box1.max.x, box2.max.x),
            y: f32::max(box1.max.y, box2.max.y),
            z: f32::max(box1.max.z, box2.max.z),
        };

        AABB {
            min: small,
            max: big
        }
    }
}