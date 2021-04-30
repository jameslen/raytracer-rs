extern crate rand; 
extern crate glam;

use rand::prelude::*;
use glam::*;

pub fn is_near_zero(v: Vec3A) -> bool {
    return v.x.abs() < f32::EPSILON && v.y.abs() < f32::EPSILON && v.z.abs() < f32::EPSILON; 
}

pub fn random() -> Vec3A {
    let mut rng = rand::thread_rng();
    Vec3A::new(rng.gen(), rng.gen(), rng.gen())
}

pub fn random_range(min: f32, max: f32) -> Vec3A {
    let mut rng = rand::thread_rng();
    glam::Vec3A::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max)
    )
}

pub fn random_in_unit_sphere() -> Vec3A {
    let mut p = random_range(-1.0, 1.0);
    while p.length_squared() >= 1.0 {
        p = random_range(-1.0, 1.0);
    }
    p
}

pub fn random_unit_vector() -> Vec3A {
    random_in_unit_sphere().normalize()
}

pub fn random_in_hemisphere(normal: Vec3A) -> Vec3A {
    let in_unit_sphere = random_in_unit_sphere();

    if in_unit_sphere.dot(normal) > 0.0 {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

pub fn random_in_unit_disk() -> Vec3A {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3A::new( 
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),  
            0.0
        );

        if p.length_squared() >= 1.0 {
            return p;
        }
    }
}

pub fn reflect(_vec: Vec3A, _normal: Vec3A) -> Vec3A {
    let scalar = 2_f32 * _vec.dot(_normal);
    let scaled_normal = scalar * _normal;
    _vec - scaled_normal
}

pub fn refract(uv: Vec3A, normal: Vec3A, etai_over_etat: f32) -> Vec3A {
    let dot_p = normal.dot(-uv);
    let cos_theta = f32::min(dot_p, 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * normal);
    let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared())) * normal;
    
    r_out_perp + r_out_parallel
}