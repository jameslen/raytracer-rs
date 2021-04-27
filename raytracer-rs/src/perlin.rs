extern crate rand;
extern crate glam;

use crate::vec3_helpers;

use glam::*;

use rand::prelude::*;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    pub rand_vec: Vec<glam::Vec3>,
    pub perm_x: Vec<i32>,
    pub perm_y: Vec<i32>,
    pub perm_z: Vec<i32>
}

impl Perlin {
    pub fn new() -> Self {
        let mut rand_vec = Vec::<Vec3>::new();
        rand_vec.reserve(POINT_COUNT);

        let mut rng = rand::thread_rng();

        for _ in 0..POINT_COUNT {
            rand_vec.push(vec3_helpers::random_range(-1.0, 1.0));
        }

        Perlin {
            rand_vec: rand_vec,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm()
        }
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p: Vec<i32> = (0..(POINT_COUNT as i32)).collect();

        let mut rng = rand::thread_rng();

        p.shuffle(&mut rng);

        return p;
    }

    pub fn noise(&self, point: Vec3) -> f32 {
        let mut u = point.x - point.x.floor();
        let mut v = point.y - point.y.floor();
        let mut w = point.z - point.z.floor();

        let i = point.x.floor() as i32;
        let j = point.y.floor() as i32;
        let k = point.z.floor() as i32;

        let mut c = [[[glam::Vec3::new(0.0,0.0,0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand_vec[
                        (self.perm_x[(i as usize + di) & 255] ^ 
                         self.perm_y[(j as usize + dj) & 255] ^ 
                         self.perm_z[(k as usize + dk) & 255]) as usize
                    ];
                }
            }
        }

        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut acc: f32 = 0.0;

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let weight = Vec3::new(u - di as f32, v - dj as f32, w - dk as f32);
                    acc += (di as f32 * uu + (1.0 - di as f32) * (1.0 - uu)) *
                           (dj as f32 * vv + (1.0 - dj as f32) * (1.0 - vv)) *
                           (dk as f32 * ww + (1.0 - dk as f32) * (1.0 - ww)) * 
                           weight.dot(c[di][dj][dk]);
                }
            }
        }

        return acc;
    }

    pub fn turb(&self, point: Vec3, depth: i32) -> f32 {
        let mut acc = 0.0;
        let mut temp = point;
        let mut weight = 1.0;
        
        for _ in 0..depth {
            acc += weight * self.noise(temp);
            weight *= 0.5;
            temp = 2.0 * temp;
        }

        return acc.abs();
    }
}
