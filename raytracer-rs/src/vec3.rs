extern crate rand; 

use rand::prelude::*;

use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3
{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3
{
    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen()
        }
    }

    pub fn random_range(min: f32, max: f32) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max)
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut p = Vec3::random_range(-1.0, 1.0);
        while p.length_sq() >= 1.0 {
            p = Vec3::random_range(-1.0, 1.0);
        }
        p
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().normalize()
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();

        if in_unit_sphere.dot(&normal) > 0.0 {
            return in_unit_sphere;
        } else {
            return -in_unit_sphere;
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3{ 
                x: rng.gen_range(-1.0..1.0),
                y: rng.gen_range(-1.0..1.0),  
                z: 0.0
            };

            if p.length_sq() >= 1.0 {
                return p;
            }
        }
    }

    pub fn cross(&self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * _rhs.z - self.z * _rhs.y,
            y: self.z * _rhs.x - self.x * _rhs.z,
            z: self.x * _rhs.y - self.y * _rhs.x
        }
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn length_sq(&self) -> f32 {
        self.dot(self)
    }

    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let length = self.length();

        self / length
    }

    pub fn is_near_zero(&self) -> bool {
        return (self.x.abs() < f32::EPSILON) && (self.y.abs() < f32::EPSILON) && (self.z.abs() < f32::EPSILON);
    }

    pub fn reflect(_vec: &Vec3, _normal: &Vec3) -> Vec3 {
        let scalar = 2_f32 * _vec.dot(&_normal);
        let scaled_normal = scalar * _normal;
        _vec - &scaled_normal
    }

    pub fn refract(uv: Vec3, normal: Vec3, etai_over_etat: f32) -> Vec3 {
        let dot_p = Vec3::dot(&-uv, &normal);
        let cos_theta = f32::min(dot_p, 1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * normal);
        let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.length_sq())) * normal;
        
        r_out_perp + r_out_parallel
    }
}

impl ops::Add for &Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Self) -> Vec3 {
        Vec3{
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z 
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Self) -> Vec3 {
        Vec3{
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z 
        }
    }
}

impl ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, _rhs: &Vec3) {
        *self = Vec3 { 
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z 
        };
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, _rhs: Vec3) {
        *self = Vec3 { 
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z 
        };
    }
}

impl ops::Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Self) -> Vec3 {
        Vec3{ x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Self) -> Vec3 {
        Vec3{ x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z }
    }
}

impl ops::SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, _rhs: &Vec3) {
        *self = Vec3 { 
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z 
        };
    }
}

impl ops::Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Vec3 {
        Vec3{
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3{
            x: _rhs.x * self.x,
            y: _rhs.y * self.y,
            z: _rhs.z * self.z
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Vec3 {
        Vec3{
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z
        }
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, _rhs: f32) {
        *self = Vec3{
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs
        }
    }
}

impl ops::Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _rhs: &Vec3) -> Vec3 {
        _rhs * self
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        _rhs * self
    }
}

impl ops::Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f32) -> Vec3 {
        self * (1_f32 / _rhs)
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f32) -> Vec3 {
        self * (1_f32 / _rhs)
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, _rhs: f32) {
        let reciprical = 1_f32 * _rhs;
        *self *= reciprical
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3{ x: -self.x, y: -self.y, z: -self.z}
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3{ x: -self.x, y: -self.y, z: -self.z}
    }
}