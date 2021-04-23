use crate::vec3::Vec3;
use crate::ray::Ray;

use std::rc::Rc;

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord, attentuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

pub struct LambertianMat {
    albedo: Vec3
}

impl LambertianMat {
    pub fn new(albedo: Vec3) -> Self {
        LambertianMat{
            albedo: albedo
        }
    }
}

impl Material for LambertianMat {
    fn scatter(&self, _ray: &Ray, record: &HitRecord, attentuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut scatter = record.normal + Vec3::random_unit_vector();

        if scatter.is_near_zero() {
            scatter = record.normal;
        }

        *scattered = Ray{
            origin: record.point,
            direction: scatter
        };
        *attentuation = self.albedo;
        return true;
    }
}

pub struct MetalMat {
    albedo: Vec3,
    fuzz: f32
}

impl MetalMat {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        MetalMat{
            albedo: albedo,
            fuzz: {
                if fuzz < 1.0 {
                    fuzz
                } else {
                    1.0
                }
            }
        }
    }
}

impl Material for MetalMat {
    fn scatter(&self, ray: &Ray, record: &HitRecord, attentuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(&ray.direction.normalize(), &record.normal);
        *scattered = Ray{
            origin: record.point,
            direction: reflected + self.fuzz * Vec3::random_in_unit_sphere()
        };
        *attentuation = self.albedo;
        return scattered.direction.dot(&record.normal) > 0.0;
    }
}

pub struct DielectricMat {
    index_refraction: f32
}

impl DielectricMat {
    pub fn new(index: f32) -> Self {
        Self {
            index_refraction: index
        }
    }
}

impl Material for DielectricMat {
    fn scatter(&self, ray: &Ray, record: &HitRecord, attentuation: &mut Vec3, scattered: &mut Ray) -> bool {
        *attentuation = Vec3{ x: 1.0, y: 1.0, z: 1.0 };
        let refraction_ratio = if record.front_face { self.index_refraction } else { self.index_refraction };

        let unit_direction = ray.direction.normalize();
        let cos_theta = f32::min(Vec3::dot(&-unit_direction, &record.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract {
            direction = Vec3::reflect(&unit_direction, &record.normal);
        } else {
            direction = Vec3::refract(unit_direction, record.normal, refraction_ratio);
        }

        *scattered = Ray{ origin: record.point, direction: direction };
        return true;
    }
}

pub struct NoMaterial {
}

impl Material for NoMaterial {
    fn scatter(&self, _ray: &Ray, _record: &HitRecord, _attentuation: &mut Vec3, _scattered: &mut Ray) -> bool {
        println!("No Material");
        return false;
    }
}

#[derive(Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub material: Rc<dyn Material>,
    pub front_face: bool
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord{
            point: Vec3{ x: 0.0, y: 0.0, z: 0.0 },
            normal: Vec3{ x: 0.0, y: 0.0, z: 0.0 },
            t: f32::INFINITY,
            material: Rc::new(NoMaterial{}),
            front_face: false
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = {
            if self.front_face {
                *outward_normal
            }
            else {
                -outward_normal
            }
        };
    }
}

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

        let root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            let root = (-half_b + sqrtd) / a;

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

pub struct AxisAlignedBox {
    pub position: Vec3,
    pub width: f32,  // X dimension
    pub height: f32, // Y dimension
    pub depth: f32,  // Z dimension
}

pub struct Plane {
    pub normal: Vec3,
    pub distance: f32
}

pub struct Triangle {
    pub p1: Vec3,
    pub p2: Vec3,
    pub p3: Vec3,
    pub normal: Vec3
}

pub enum Shape {
    Sphere(Sphere),
    Box(AxisAlignedBox),
    Plane(Plane),
    Triangle(Triangle)
}