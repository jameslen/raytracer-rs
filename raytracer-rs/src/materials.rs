use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hit_record::HitRecord;

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord, attentuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

#[derive(Copy, Clone)]
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
            direction: scatter,
            time: _ray.time
        };
        *attentuation = self.albedo;
        return true;
    }
}

#[derive(Copy, Clone)]
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
            direction: reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            time: ray.time
        };
        *attentuation = self.albedo;
        return scattered.direction.dot(&record.normal) > 0.0;
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
    fn scatter(&self, ray: &Ray, record: &HitRecord, attentuation: &mut Vec3, scattered: &mut Ray) -> bool {
        *attentuation = Vec3{ x: 1.0, y: 1.0, z: 1.0 };
        let refraction_ratio: f32;
        
        if record.front_face { 
            refraction_ratio = 1.0 / self.index_refraction;
        } else { 
            refraction_ratio = self.index_refraction;
        };

        let unit_direction = ray.direction.normalize();
        let cos_theta = f32::min(Vec3::dot(&-unit_direction, &record.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || self.reflectance(cos_theta, refraction_ratio) > rand::random() {
            direction = Vec3::reflect(&unit_direction, &record.normal);
        } else {
            direction = Vec3::refract(unit_direction, record.normal, refraction_ratio);
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
    fn scatter(&self, _ray: &Ray, _record: &HitRecord, _attentuation: &mut Vec3, _scattered: &mut Ray) -> bool {
        println!("No Material");
        return false;
    }
}