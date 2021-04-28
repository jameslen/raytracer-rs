extern crate glam;

use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::texture::*;

use crate::vec3_helpers;

use std::sync::Arc;

use glam::*;

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord, attentuation: &mut Vec3A, scattered: &mut Ray) -> bool;
    fn emitted(&self, _tex_coords: (f32, f32), _point: Vec3A) -> Vec3A {
        Vec3A::ZERO
    }
}

#[derive(Clone)]
pub struct LambertianMat {
    albedo: Arc<dyn Texture>
}

impl LambertianMat {
    pub fn from_texture<T: 'static + Texture>(albedo: T) -> Self {
        LambertianMat{
            albedo: Arc::new(albedo)
        }
    }

    pub fn from_shared_texture(albedo: Arc<dyn Texture>) -> Self {
        LambertianMat {
            albedo: albedo
        }
    }

    pub fn from_color(albedo: Vec3A) -> Self {
        LambertianMat{
            albedo: Arc::new(SolidColor{color: albedo})
        }
    }
}

impl Material for LambertianMat {
    fn scatter(&self, _ray: &Ray, record: &HitRecord, attentuation: &mut Vec3A, scattered: &mut Ray) -> bool {
        let mut scatter = record.normal + vec3_helpers::random_unit_vector();

        if vec3_helpers::is_near_zero(scatter) {
            scatter = record.normal;
        }

        *scattered = Ray{
            origin: record.point,
            direction: scatter,
            time: _ray.time
        };
        *attentuation = self.albedo.value(record.tex_coords, record.point);
        return true;
    }
}

#[derive(Copy, Clone)]
pub struct MetalMat {
    albedo: Vec3A,
    fuzz: f32
}

impl MetalMat {
    pub fn new(albedo: Vec3A, fuzz: f32) -> Self {
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
    fn scatter(&self, ray: &Ray, record: &HitRecord, attentuation: &mut Vec3A, scattered: &mut Ray) -> bool {
        let reflected = vec3_helpers::reflect(ray.direction.normalize(), record.normal);
        *scattered = Ray{
            origin: record.point,
            direction: reflected + self.fuzz * vec3_helpers::random_in_unit_sphere(),
            time: ray.time
        };
        *attentuation = self.albedo;
        return scattered.direction.dot(record.normal) > 0.0;
    }
}

#[derive(Copy, Clone)]
pub struct DielectricMat {
    index_refraction: f32
}

impl DielectricMat {
    pub fn new(index: f32) -> Self {
        Self {
            index_refraction: index
        }
    }

    fn reflectance(&self, cosine: f32, ref_index: f32) -> f32 {
        let mut r0 = (1.0 - ref_index) / (1.0 + ref_index);
        r0 = r0 * r0;

        return r0 + (1.0 - r0) * f32::powf(1.0 - cosine, 5.0); 
    }
}

impl Material for DielectricMat {
    fn scatter(&self, ray: &Ray, record: &HitRecord, attentuation: &mut Vec3A, scattered: &mut Ray) -> bool {
        *attentuation = Vec3A::ONE;
        let refraction_ratio: f32;
        
        if record.front_face { 
            refraction_ratio = 1.0 / self.index_refraction;
        } else { 
            refraction_ratio = self.index_refraction;
        };

        let unit_direction = ray.direction.normalize();
        let cos_theta = f32::min(record.normal.dot(-unit_direction), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3A;

        if cannot_refract || self.reflectance(cos_theta, refraction_ratio) > rand::random() {
            direction = vec3_helpers::reflect(unit_direction, record.normal);
        } else {
            direction = vec3_helpers::refract(unit_direction, record.normal, refraction_ratio);
        }

        *scattered = Ray{ 
            origin: record.point, 
            direction: direction,
            time: ray.time
        };
        return true;
    }
}

#[derive(Copy, Clone)]
pub struct NoMaterial {
}

impl Material for NoMaterial {
    fn scatter(&self, _ray: &Ray, _record: &HitRecord, _attentuation: &mut Vec3A, _scattered: &mut Ray) -> bool {
        println!("No Material");
        return false;
    }
}

pub struct DiffuseLight {
    emit: Arc<dyn Texture>
}

impl DiffuseLight {
    pub fn from_texture<T: 'static + Texture>(texture: T) -> Self {
        DiffuseLight {
            emit: Arc::new(texture)
        }
    }
    pub fn from_color(color: Vec3A) -> Self {
        DiffuseLight::from_texture(SolidColor{color:color})
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _record: &HitRecord, _attentuation: &mut Vec3A, _scattered: &mut Ray) -> bool {
        return false;
    }

    fn emitted(&self, tex_coords: (f32, f32), point: Vec3A) -> Vec3A {
        self.emit.value(tex_coords, point)
    }
}