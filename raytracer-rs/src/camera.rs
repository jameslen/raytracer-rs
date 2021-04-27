extern crate glam;

use crate::vec3_helpers;

use glam::*;

use crate::ray::Ray;


extern crate rand; 

use rand::prelude::*;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
    time_start: f32,
    time_end: f32
}

impl Camera {
    pub fn new(origin: Vec3, target: Vec3, up: Vec3, vfov: f32, aspect_ratio: f32, aperture: f32, focus_distance: f32, time_start: f32, time_end: f32) -> Camera {
        let h = f32::tan(vfov / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (origin - target).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);
        
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;

        Camera {
            origin: origin,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - focus_distance * w,
            horizontal: horizontal,
            vertical: vertical,
            u: u,
            v: v,
            w: w,
            lens_radius: aperture / 2.0,
            time_start: time_start,
            time_end: time_end
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * vec3_helpers::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        let mut rng = rand::thread_rng();

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            time: rng.gen_range(self.time_start..self.time_end)
        }
    }
}