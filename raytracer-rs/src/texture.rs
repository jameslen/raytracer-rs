extern crate glam;
extern crate image;

use glam::*;
use image::*;

use crate::perlin::Perlin;

use std::rc::Rc;

pub trait Texture {
    fn value(&self, coords: (f32, f32), point: Vec3A) -> Vec3A;
}

pub struct SolidColor {
    pub color: Vec3A
}

impl Texture for SolidColor {
    fn value(&self, _coords: (f32, f32), _point: Vec3A) -> Vec3A {
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

    pub fn from_color(odd: Vec3A, even: Vec3A) -> Self {
        CheckeredTexture{
            odd: Rc::new(SolidColor{color: odd}),
            even: Rc::new(SolidColor{color: even})
        }
    }
}

impl Texture for CheckeredTexture {
    fn value(&self, coords: (f32, f32), point: Vec3A) -> Vec3A {
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
    fn value(&self, _coords: (f32, f32), point: Vec3A) -> Vec3A {
        return Vec3A::new(0.5, 0.5, 0.5) * (1.0 + f32::sin(self.frequency * point.z + 10.0 * self.noise.turb(point, 7)));
    }
}

pub struct ImageTexture {
    image: image::RgbImage
}

impl ImageTexture {
    pub fn new(path: String) -> Self {
        ImageTexture {
            image: image::open(path).unwrap().into_rgb8()
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, _coords: (f32, f32), _point: Vec3A) -> Vec3A {
        let u = f32::clamp(_coords.0, 0.0, 1.0);
        let v = 1.0 - f32::clamp(_coords.1, 0.0, 1.0);

        let (width, height) = self.image.dimensions();

        let mut i = (u * width as f32) as u32;
        let mut j = (v * height as f32) as u32;

        if i >= width {
            i = width - 1;
        }

        if j >= height {
            j = height - 1;
        }

        let color = self.image.get_pixel(i, j);

        Vec3A::new(color.0[0] as f32 / 255.0, color.0[1] as f32 / 255.0, color.0[2] as f32 / 255.0)
    }
}