extern crate rand;

use crate::vec3::Vec3;

use rand::prelude::*;

const POINT_COUNT = 256;

pub struct Perlin {
    pub rand_float: Vec<f32>,
    pub perm_x: Vec<i32>,
    pub perm_y: Vec<i32>,
    pub perm_z: Vec<i32>
}

impl Perlin {
    pub fn new() -> Self {
        let rand_float = Vec::<f32>::new();
        rand_float.reserve(POINT_COUNT);

        let mut rng = rand::thread_rng();

        for i in 0..POINT_COUNT {
            rand_float.push(rng.gen());
        }

        Perlin {
            rand_float: rand_float,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm()
        }
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = vec![0..POINT_COUNT];

        let mut rng = rand::thread_rng();

        p.shuffle(rng);

        return p;
    }
}
