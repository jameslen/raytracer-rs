use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::materials::Material;

use std::rc::Rc;

pub trait Hittable {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Rc::<dyn Material>
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
}

pub struct MovingSphere {
    pub center_start: Vec3,
    pub center_end: Vec3,
    pub radius: f32,
    pub time_start: f32,
    pub time_end: f32,
    pub material: Rc::<dyn Material>
}

impl MovingSphere {
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