use crate::vec3::Vec3;
use crate::perlin::Perlin;

use std::rc::Rc;

pub trait Texture {
    fn value(&self, coords: (f32, f32), point: Vec3) -> Vec3;
}

pub struct SolidColor {
    pub color: Vec3
}

impl Texture for SolidColor {
    fn value(&self, _coords: (f32, f32), _point: Vec3) -> Vec3 {
        return self.color;
    }
}

pub struct CheckeredTexture {
    pub odd: Rc<dyn Texture>,
    pub even: Rc<dyn Texture>
}

impl CheckeredTexture {
    pub fn from_texture<S: 'static + Texture, T: 'static + Texture>(odd: S, even: T) -> Self {
        CheckeredTexture{
            odd: Rc::new(odd),
            even: Rc::new(even)
        }
    }

    pub fn from_shared_texture(odd: Rc<dyn Texture>, even: Rc<dyn Texture>) -> Self {
        CheckeredTexture{
            odd: odd.clone(),
            even: even.clone()
        }
    }

    pub fn from_color(odd: Vec3, even: Vec3) -> Self {
        CheckeredTexture{
            odd: Rc::new(SolidColor{color: odd}),
            even: Rc::new(SolidColor{color: even})
        }
    }
}

impl Texture for CheckeredTexture {
    fn value(&self, coords: (f32, f32), point: Vec3) -> Vec3 {
        let sines = f32::sin(10.0 * point.x) * f32::sin(10.0 * point.y) * f32::sin(10.0 * point.z);

        if sines < 0.0 {
            return self.odd.value(coords, point);
        } else {
            return self.even.value(coords, point);
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    frequency: f32
}

impl NoiseTexture {
    pub fn new(freq: f32) -> Self {
        Self {
            noise: Perlin::new(),
            frequency: freq
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, coords: (f32, f32), point: Vec3) -> Vec3 {
        return Vec3{x: 0.5, y: 0.5, z: 0.5} * (1.0 + self.noise.noise(self.frequency * point));
    }
}