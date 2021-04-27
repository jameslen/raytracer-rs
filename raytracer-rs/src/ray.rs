extern crate glam;

use glam::Vec3;

pub struct Ray
{
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f32
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + (t * self.direction)
    }
}