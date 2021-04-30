extern crate glam;

use glam::Vec3A;

pub struct Ray
{
    pub origin: Vec3A,
    pub direction: Vec3A,
    pub time: f32
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3A {
        self.origin + (t * self.direction)
    }
}