use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(origin: Vec3, viewport_height: f32, aspect_ratio: f32, focal_length: f32) -> Camera {
        let viewport_width = viewport_height * aspect_ratio;
        let horizontal = Vec3{ x: viewport_width, y: 0.0, z: 0.0 };
        let vertical = Vec3{ x: 0.0, y: viewport_height, z: 0.0 };
        
        Camera {
            origin: origin,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - Vec3{ x: 0.0, y: 0.0, z: focal_length },
            horizontal: horizontal,
            vertical: vertical
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin
        }
    }
}