use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::materials::Material;
use crate::aabb::AABB;

use std::rc::Rc;

pub trait Hittable {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
    fn bounding_box(&self, t0: f32, t1: f32, aabb: &mut AABB) -> bool;
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Rc::<dyn Material>
}

impl Sphere {
    pub fn new<T: 'static + Material>(center: Vec3, radius: f32, material: T) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: Rc::new(material)
        }
    }
}

impl Hittable for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_sq();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_sq() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;

            if root < t_min || root > t_max {
                return false;
            }
        }

        let outward_normal = (ray.at(root) - self.center) / self.radius;

        *record = HitRecord{
            t: root,
            point: ray.at(root),
            normal: outward_normal,
            material: self.material.clone(),
            front_face: true
        };

        record.set_face_normal(&ray, &outward_normal);

        return true;
    }

    fn bounding_box(&self, _t0: f32, _t1: f32, aabb: &mut AABB) -> bool {
        let offset = Vec3{ x: self.radius, y: self.radius, z: self.radius };
        *aabb = AABB {
            min: self.center - offset,
            max: self.center + offset
        };
        return true;
    }
}

#[derive(Clone)]
pub struct MovingSphere {
    pub center_start: Vec3,
    pub center_end: Vec3,
    pub radius: f32,
    pub time_start: f32,
    pub time_end: f32,
    pub material: Rc::<dyn Material>
}

impl MovingSphere {
    pub fn new<T: 'static + Material>(center_start: Vec3, center_end: Vec3, radius: f32, time_start: f32, time_end: f32, material: T) -> MovingSphere {
        MovingSphere {
            center_start: center_start,
            center_end: center_end,
            time_start: time_start,
            time_end: time_end,
            radius: radius,
            material: Rc::new(material)
        }
    }

    pub fn center(&self, time: f32) -> Vec3 {
        return self.center_start + ((time - self.time_start) / (self.time_end - self.time_start) * (self.center_end - self.center_start));
    }
}

impl Hittable for MovingSphere {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.length_sq();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_sq() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;

            if root < t_min || root > t_max {
                return false;
            }
        }

        let outward_normal = (ray.at(root) - self.center(ray.time)) / self.radius;

        *record = HitRecord{
            t: root,
            point: ray.at(root),
            normal: outward_normal,
            material: self.material.clone(),
            front_face: true
        };

        record.set_face_normal(&ray, &outward_normal);

        return true;
    }

    fn bounding_box(&self, t0: f32, t1: f32, aabb: &mut AABB) -> bool {
        let offset = Vec3{ x: self.radius, y: self.radius, z: self.radius };
        let start = AABB {
            min: self.center(t0) - offset,
            max: self.center(t0) + offset
        };

        let end = AABB {
            min: self.center(t1) - offset,
            max: self.center(t1) + offset
        };

        *aabb = AABB::surrounding_box(&start, &end);

        return true;
    }
}

// pub struct AxisAlignedBox {
//     pub position: Vec3,
//     pub width: f32,  // X dimension
//     pub height: f32, // Y dimension
//     pub depth: f32,  // Z dimension
// }

// pub struct Plane {
//     pub normal: Vec3,
//     pub distance: f32
// }

// pub struct Triangle {
//     pub p1: Vec3,
//     pub p2: Vec3,
//     pub p3: Vec3,
//     pub normal: Vec3
// }