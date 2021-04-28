extern crate glam;

use glam::*;

use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::materials::*;//Material;
use crate::aabb::AABB;

use crate::texture::*;

use std::rc::Rc;

pub trait Hittable {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}

fn get_sphere_uv(point: Vec3A) -> (f32, f32) {
    let theta = f32::acos(-point.y);
    let phi = f32::atan2(-point.z, point.x) + std::f32::consts::PI;

    (phi / (2.0 * std::f32::consts::PI), theta / std::f32::consts::PI)
}

pub struct TransformedObject<T: Hittable> {
    object: T,
    transform: Mat4,
    inv_transform: Mat4,
    aabb: AABB
}

impl<T: Hittable> TransformedObject<T> {
    pub fn new(object: T, transform: Mat4) -> Self {
        Self{
            aabb: Self::generate_aabb(&object, transform),
            object: object,
            transform: transform,
            inv_transform: transform.inverse()
        }
    }

    fn generate_aabb(object: &T, transform: Mat4) -> AABB {
        let base_aabb = object.bounding_box(0.0, 1.0).unwrap();
        println!("pre: min: {:?}, max: {:?}", base_aabb.min, base_aabb.max);
        let min = transform.transform_point3a(base_aabb.min);
        let max = transform.transform_point3a(base_aabb.max);
        println!("pre: min: {:?}, max: {:?}", min, max);

        AABB{
            min: min.min(max),
            max: max.max(min)
        }
    }
}

impl<T: Hittable> Hittable for TransformedObject<T> {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let local_ray = Ray{ 
            origin: self.inv_transform.transform_point3a(ray.origin), 
            direction: self.inv_transform.transform_vector3a(ray.direction),
            time: ray.time
        };

        let result = self.object.intersect(&local_ray, t_min, t_max);

        if let Some(record) = result {
            let normal = self.transform.transform_vector3a(record.normal).normalize();
            let point = self.transform.transform_point3a(record.point);
            let mut record = HitRecord{
                point: point, //ray.at(record.t),
                t: record.t,
                normal: normal,
                material: record.material,
                tex_coords: record.tex_coords,
                front_face: record.front_face
            };

            record.set_face_normal(ray, &normal);

            return Some(record); 
        }

        return None;
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        return Some(self.aabb);
    }
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3A,
    pub radius: f32,
    pub material: Rc::<dyn Material>
}

impl Sphere {
    pub fn new<T: 'static + Material>(center: Vec3A, radius: f32, material: T) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: Rc::new(material)
        }
    }
}

impl Hittable for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return Option::None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;

            if root < t_min || root > t_max {
                return Option::None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;

        let mut record = HitRecord{
            t: root,
            point: point,
            normal: outward_normal,
            material: self.material.clone(),
            tex_coords: get_sphere_uv(outward_normal),
            front_face: true
        };

        record.set_face_normal(&ray, &outward_normal);

        return Option::Some(record);
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let offset = Vec3A::new(self.radius, self.radius, self.radius);
        return Option::Some(AABB {
            min: self.center - offset,
            max: self.center + offset
        });
    }
}

#[derive(Clone)]
pub struct MovingSphere {
    pub center_start: Vec3A,
    pub center_end: Vec3A,
    pub radius: f32,
    pub time_start: f32,
    pub time_end: f32,
    pub material: Rc::<dyn Material>
}

impl MovingSphere {
    pub fn new<T: 'static + Material>(center_start: Vec3A, center_end: Vec3A, radius: f32, time_start: f32, time_end: f32, material: T) -> MovingSphere {
        MovingSphere {
            center_start: center_start,
            center_end: center_end,
            time_start: time_start,
            time_end: time_end,
            radius: radius,
            material: Rc::new(material)
        }
    }

    pub fn center(&self, time: f32) -> Vec3A {
        return self.center_start + ((time - self.time_start) / (self.time_end - self.time_start) * (self.center_end - self.center_start));
    }
}

impl Hittable for MovingSphere {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return Option::None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;

            if root < t_min || root > t_max {
                return Option::None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center(ray.time)) / self.radius;

        let mut record = HitRecord{
            t: root,
            point: ray.at(root),
            normal: outward_normal,
            material: self.material.clone(),
            tex_coords: get_sphere_uv(outward_normal),
            front_face: true
        };

        record.set_face_normal(&ray, &outward_normal);

        return Option::Some(record);
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let offset = Vec3A::new(self.radius, self.radius, self.radius);
        let start = AABB {
            min: self.center(t0) - offset,
            max: self.center(t0) + offset
        };

        let end = AABB {
            min: self.center(t1) - offset,
            max: self.center(t1) + offset
        };

        return Some(AABB::surrounding_box(&start, &end))
    }
}

pub struct XYRect {
    min: Vec2,
    max: Vec2,
    offset: f32,
    pub material: Rc::<dyn Material>
}

impl XYRect {
    pub fn new<T: 'static + Material>(min: Vec2, max: Vec2, offset: f32, material: T) -> Self {
        XYRect {
            min: min,
            max: max,
            offset: offset,
            material: Rc::new(material)
        }
    }
}

impl Hittable for XYRect {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.offset - ray.origin.z) / ray.direction.z;

        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;

        if x < self.min.x || x > self.max.x || y < self.min.y || y > self.max.y {
            return None;
        }

        let mut record = HitRecord {
            t: t,
            point: ray.at(t),
            tex_coords: ((x - self.min.x) / (self.max.x - self.min.x), (y - self.min.y) / (self.max.y - self.min.y)),
            normal: Vec3A::Z,
            material: self.material.clone(),
            front_face: true
        };

        record.set_face_normal(ray, &Vec3A::Z);

        return Some(record);
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB {
            min: Vec3A::new(self.min.x, self.min.y, self.offset - 0.0001),
            max: Vec3A::new(self.max.x, self.max.y, self.offset + 0.0001),
        })
    }
}

pub struct XZRect {
    min: Vec2,
    max: Vec2,
    offset: f32,
    pub material: Rc::<dyn Material>
}

impl XZRect {
    pub fn new<T: 'static + Material>(min: Vec2, max: Vec2, offset: f32, material: T) -> Self {
        XZRect {
            min: min,
            max: max,
            offset: offset,
            material: Rc::new(material)
        }
    }
}

impl Hittable for XZRect {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.offset - ray.origin.y) / ray.direction.y;

        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;

        if x < self.min.x || x > self.max.x || z < self.min.y || z > self.max.y {
            return None;
        }

        let mut record = HitRecord {
            t: t,
            point: ray.at(t),
            tex_coords: ((x - self.min.x) / (self.max.x - self.min.x), (z - self.min.y) / (self.max.y - self.min.y)),
            normal: Vec3A::Y,
            material: self.material.clone(),
            front_face: true
        };

        record.set_face_normal(ray, &Vec3A::Y);

        return Some(record);
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB {
            min: Vec3A::new(self.min.x, self.offset - 0.0001, self.min.y),
            max: Vec3A::new(self.max.x, self.offset + 0.0001, self.max.y),
        })
    }
}

pub struct YZRect {
    min: Vec2,
    max: Vec2,
    offset: f32,
    pub material: Rc::<dyn Material>
}

impl YZRect {
    pub fn new<T: 'static + Material>(min: Vec2, max: Vec2, offset: f32, material: T) -> Self {
        YZRect {
            min: min,
            max: max,
            offset: offset,
            material: Rc::new(material)
        }
    }
}

impl Hittable for YZRect {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.offset - ray.origin.x) / ray.direction.x;

        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin.y + t * ray.direction.y;
        let y = ray.origin.z + t * ray.direction.z;

        if x < self.min.x || x > self.max.x || y < self.min.y || y > self.max.y {
            return None;
        }

        let mut record = HitRecord {
            t: t,
            point: ray.at(t),
            tex_coords: ((x - self.min.x) / (self.max.x - self.min.x), (y - self.min.y) / (self.max.y - self.min.y)),
            normal: Vec3A::X,
            material: self.material.clone(),
            front_face: true
        };

        record.set_face_normal(ray, &Vec3A::X);

        return Some(record);
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB {
            min: Vec3A::new(self.offset - 0.0001, self.min.x, self.min.y),
            max: Vec3A::new(self.offset + 0.0001, self.max.x, self.max.y),
        })
    }
}

pub struct Box {
    min: Vec3A,
    max: Vec3A,
    pub material: Rc::<dyn Material>
}

impl Box {
    pub fn new<T: 'static + Material>(width: f32, height: f32, depth: f32, material: T) -> Self {
        Box{ 
            min: Vec3A::ZERO,
            max: Vec3A::new(width, height, depth),
            material: Rc::new(material)
        }
    }

    pub fn testing_box<T: 'static + Material>(min: Vec3A, max: Vec3A, color: T) -> Self{
        Box{
            min: min,
            max: max,
            material: Rc::new(color)
        }
    }
}

fn axis_min(first: (f32, usize), second: (f32, usize)) -> (f32, usize) {
    if first.0 < second.0 {
        return first;
    } else {
        return second;
    }
}

fn axis_max(first: (f32, usize), second: (f32, usize)) -> (f32, usize) {
    if first.0 > second.0 {
        return first;
    } else {
        return second;
    }
}

impl Hittable for Box {
    fn intersect(&self, ray: &Ray, _t_min: f32, _t_max: f32) -> Option<HitRecord> {
        let recip = ray.direction.recip();
        let min = (self.min - ray.origin) * recip;
        let max = (self.max - ray.origin) * recip;

        let (t_min, min_axis) = axis_max(axis_max(axis_min((min.x, 0), (max.x, 1)), axis_min((min.y, 2), (max.y, 3))), axis_min((min.z, 4), (max.z, 5)));
        let (t_max, _)        = axis_min(axis_min(axis_max((min.x, 0), (max.x, 1)), axis_max((min.y, 2), (max.y, 3))), axis_max((min.z, 4), (max.z, 5)));

        if t_max < 0.0 || t_min > t_max {
            return None;
        }
        
        let normal = match min_axis {
            0 => -Vec3A::X,
            1 =>  Vec3A::X,
            2 => -Vec3A::Y,
            3 =>  Vec3A::Y,
            4 => -Vec3A::Z,
            5 =>  Vec3A::Z,
            _ =>  {
                println!("WTF");
                Vec3A::ZERO
            }
        };

        let color = match min_axis {
            0 => Vec3A::X + Vec3A::Y,
            1 =>  Vec3A::X,
            2 => Vec3A::Y + Vec3A::Z,
            3 =>  Vec3A::Y,
            4 =>  Vec3A::Z + Vec3A::X,
            5 =>  Vec3A::Z,
            _ =>  {
                println!("WTF");
                Vec3A::ZERO
            }
        };

        let point = ray.at(t_min);
        let delta = point / self.max;
        let tex_coords = match min_axis {
            0 => { // X_MIN
                (delta.y, delta.z)
            },
            1 => { // X_MAX
                (delta.y, delta.z)
            },
            2 => { // Y_MIN
                (delta.x, delta.z)
            },
            3 => { // Y_MAX
                (delta.x, delta.z)
            },
            4 => { // Z_MIN
                (delta.x, delta.y)
            },
            5 => { // Z_MAX
                (delta.x, delta.y)
            },
            _ =>  {
                println!("WTF");
                (0.0, 0.0)
            }
        };



        let mut record = HitRecord{
            t: t_min,
            point: point,
            tex_coords: tex_coords, // TODO: Impl tex coords
            normal: normal,
            //material: self.material.clone(),
            material: Rc::new(LambertianMat::from_texture(SolidColor{ color: Vec3A::new(tex_coords.0, tex_coords.1, 0.0) })),
            front_face: true
        };

        record.set_face_normal(ray, &normal);

        return Some(record);
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB{
            min: self.min,
            max: self.max
        })
    }
}
