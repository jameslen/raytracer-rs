extern crate glam;

use glam::*;

use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::materials::*;//Material;
use crate::aabb::AABB;
use crate::scene::Scene;

use std::sync::Arc;
use rand::prelude::*;

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
        let min = base_aabb.min;
        let max = base_aabb.max; 
        let p0 = transform.transform_point3a(min);
        let p1 = transform.transform_point3a(Vec3A::new(max.x, min.y, min.z));
        let p2 = transform.transform_point3a(Vec3A::new(max.x, min.y, max.z));
        let p3 = transform.transform_point3a(Vec3A::new(min.x, min.y, max.z));

        let p4 = transform.transform_point3a(Vec3A::new(min.x, max.y, min.z));
        let p5 = transform.transform_point3a(Vec3A::new(max.x, max.y, min.z));
        let p6 = transform.transform_point3a(max);
        let p7 = transform.transform_point3a(Vec3A::new(min.x, max.y, max.z));

        AABB{
            min: p0.min(p1.min(p2.min(p3.min(p4.min(p5.min(p6.min(p7))))))),
            max: p0.max(p1.max(p2.max(p3.max(p4.max(p5.max(p6.max(p7))))))),
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
    pub material: Arc::<dyn Material>
}

impl Sphere {
    pub fn new<T: 'static + Material>(center: Vec3A, radius: f32, material: T) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: Arc::new(material)
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
    pub material: Arc::<dyn Material>
}

impl MovingSphere {
    pub fn new<T: 'static + Material>(center_start: Vec3A, center_end: Vec3A, radius: f32, time_start: f32, time_end: f32, material: T) -> MovingSphere {
        MovingSphere {
            center_start: center_start,
            center_end: center_end,
            time_start: time_start,
            time_end: time_end,
            radius: radius,
            material: Arc::new(material)
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
    pub material: Arc::<dyn Material>
}

impl XYRect {
    pub fn new<T: 'static + Material>(min: Vec2, max: Vec2, offset: f32, material: T) -> Self {
        XYRect {
            min: min,
            max: max,
            offset: offset,
            material: Arc::new(material)
        }
    }

    pub fn new_with_material(min: Vec2, max: Vec2, offset: f32, material: Arc::<dyn Material>) -> Self {
        Self {
            min: min,
            max: max,
            offset: offset,
            material: material.clone()
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

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
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
    pub material: Arc::<dyn Material>
}

impl XZRect {
    pub fn new<T: 'static + Material>(min: Vec2, max: Vec2, offset: f32, material: T) -> Self {
        XZRect {
            min: min,
            max: max,
            offset: offset,
            material: Arc::new(material)
        }
    }

    pub fn new_with_material(min: Vec2, max: Vec2, offset: f32, material: Arc::<dyn Material>) -> Self {
        Self {
            min: min,
            max: max,
            offset: offset,
            material: material.clone()
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

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
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
    pub material: Arc::<dyn Material>
}

impl YZRect {
    pub fn new<T: 'static + Material>(min: Vec2, max: Vec2, offset: f32, material: T) -> Self {
        YZRect {
            min: min,
            max: max,
            offset: offset,
            material: Arc::new(material)
        }
    }

    pub fn new_with_material(min: Vec2, max: Vec2, offset: f32, material: Arc::<dyn Material>) -> Self {
        Self {
            min: min,
            max: max,
            offset: offset,
            material: material.clone()
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

pub struct Box2 {
    min: Vec3A,
    max: Vec3A,
    sides: Scene,
    pub material: Arc::<dyn Material>
}

impl Box2 {
    pub fn new<T: 'static + Material>(width: f32, height: f32, depth: f32, material: T) -> Self {
        Box2::full_box(Vec3A::ZERO, Vec3A::new(width, height, depth), Arc::new(material))
    }

    pub fn full_box(min: Vec3A, max: Vec3A, color: Arc<dyn Material>) -> Self{
        let mut sides = Scene::new();

        sides.add_shape(XYRect::new_with_material(Vec2::new(min.x, min.y), Vec2::new(max.x, max.y), min.z, color.clone()));
        sides.add_shape(XYRect::new_with_material(Vec2::new(min.x, min.y), Vec2::new(max.x, max.y), max.z, color.clone()));

        sides.add_shape(XZRect::new_with_material(Vec2::new(min.x, min.z), Vec2::new(max.x, max.z), min.y, color.clone()));
        sides.add_shape(XZRect::new_with_material(Vec2::new(min.x, min.z), Vec2::new(max.x, max.z), max.y, color.clone()));

        sides.add_shape(YZRect::new_with_material(Vec2::new(min.y, min.z), Vec2::new(max.y, max.z), min.x, color.clone()));
        sides.add_shape(YZRect::new_with_material(Vec2::new(min.y, min.z), Vec2::new(max.y, max.z), max.x, color.clone()));

        Self {
            min: min,
            max: max,
            sides: sides,
            material: color.clone()
        }
    }
}

impl Hittable for Box2 {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.intersect(ray, t_min, t_max)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB{
            min: self.min,
            max: self.max
        })
    }
}

pub struct Box {
    min: Vec3A,
    max: Vec3A,
    pub material: Arc::<dyn Material>
}

impl Box {
    pub fn new<T: 'static + Material>(width: f32, height: f32, depth: f32, material: T) -> Self {
        Box{ 
            min: Vec3A::ZERO,
            max: Vec3A::new(width, height, depth),
            material: Arc::new(material)
        }
    }

    pub fn full_box(min: Vec3A, max: Vec3A, color: Arc<dyn Material>) -> Self{
        Box{
            min: min,
            max: max,
            material: color.clone()
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
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let recip = ray.direction.recip();
        let min = (self.min - ray.origin) * recip;
        let max = (self.max - ray.origin) * recip;

        let (mut t_min0, mut min_axis) = axis_max(axis_max(axis_min((min.x, 0), (max.x, 1)), axis_min((min.y, 2), (max.y, 3))), axis_min((min.z, 4), (max.z, 5)));
        let (t_max0, max_axis)         = axis_min(axis_min(axis_max((min.x, 0), (max.x, 1)), axis_max((min.y, 2), (max.y, 3))), axis_max((min.z, 4), (max.z, 5)));

        if t_max0 <= 0.0 || t_min0 > t_max0 || t_min0 > t_max {
            return None;
        }

        // if t_min0 < t_min {
        //     t_min0 = t_max0;
        //     min_axis = max_axis;
        // }
        
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

        // let color = match min_axis {
        //     0 => Vec3A::X + Vec3A::Y,
        //     1 =>  Vec3A::X,
        //     2 => Vec3A::Y + Vec3A::Z,
        //     3 =>  Vec3A::Y,
        //     4 =>  Vec3A::Z + Vec3A::X,
        //     5 =>  Vec3A::Z,
        //     _ =>  {
        //         println!("WTF");
        //         Vec3A::ZERO
        //     }
        // };

        let point = ray.at(t_min0);
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
            t: t_min0,
            point: point,
            tex_coords: tex_coords, // TODO: Impl tex coords
            normal: normal,
            material: self.material.clone(),
            //material: Arc::new(LambertianMat::from_texture(SolidColor{ color: color })),
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

pub struct ConstantMedium<T: Hittable> {
    boundary: T,
    negative_density: f64,
    material: Arc<dyn Material>
}

impl<T: Hittable> ConstantMedium<T> {
    // pub fn new(boundary: T, density: f32, texture: Arc<dyn Texture>) -> Self{
    //     Self{
    //         boundary: boundary,
    //         negative_density: -1.0 / density,
    //         material: Arc::new(IsotropicMat::new(texture.clone()))
    //     }
    // }

    pub fn from_color(boundary: T, density: f32, color: Vec3A) -> Self{
        Self{
            boundary: boundary,
            negative_density: -1.0 / density as f64,
            material: Arc::new(IsotropicMat::from_color(color))
        }
    }
}

impl<T: Hittable> Hittable for ConstantMedium<T> {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(mut record) = self.boundary.intersect(ray, -f32::INFINITY, f32::INFINITY) {
            if let Some(mut record2) = self.boundary.intersect(ray, record.t + 0.0001, f32::INFINITY) {
                let mut rng = rand::thread_rng();
                let enable_debug = false;
                let debugging = enable_debug && rng.gen::<f32>() < 0.00001;

                if debugging {
                    println!("t_min = {}, t_max = {}", record.t, record2.t);
                }

                if record.t < t_min {
                    record.t = t_min;
                }
                if record2.t > t_max {
                    record2.t = t_max;
                }

                if record.t >= record2.t {
                    return None;
                }

                if record.t < 0.0 {
                    record.t = 0.0;
                }

                let length = ray.direction.length() as f64;
                let distance_inside = (record2.t - record.t) as f64 / length;
                let hit_distance = self.negative_density * f64::ln(rng.gen_range(0.0..1.0));

                if hit_distance > distance_inside {
                    return None;
                }

                let final_t = record.t + (hit_distance / length) as f32;
                let final_point = ray.at(final_t);

                if debugging {
                    println!("distance = {},\nt = {}, \np: {:?}", hit_distance, final_t, final_point);
                }

                return Some(HitRecord{
                    t: final_t,
                    point: final_point,
                    normal: Vec3A::X,
                    front_face: true,
                    material: self.material.clone(),
                    tex_coords: (0.0, 0.0)
                });
            }
        }
        return None;
    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        self.boundary.bounding_box(_t0, _t1)
    }
}