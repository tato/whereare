use bevy::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};
use rand::{Rng, SeedableRng};
pub struct NoiseMap {
    width: u32,
    _height: u32,
    noise_map: Vec<f64>,
}

impl NoiseMap {
    pub fn empty() -> NoiseMap {
        NoiseMap {
            width: 1,
            _height: 1,
            noise_map: vec![0.0],
        }
    }
    pub fn generate(
        width: u32,
        height: u32,
        seed: u32,
        mut scale: f64,
        octaves: u32,
        persistance: f64,
        lacunarity: f64,
        offset: Vec2,
    ) -> NoiseMap {
        let perlin = Perlin::new().set_seed(seed);
        let mut prng = rand_chacha::ChaCha8Rng::seed_from_u64(u64::from(seed));

        let octave_offsets: Vec<Vec2> = (0..octaves)
            .map(|_i| {
                let offset_x = prng.gen_range(-100000.0, 100000.0) + offset.x;
                let offset_y = prng.gen_range(-100000.0, 100000.0) + offset.y;
                Vec2::new(offset_x, offset_y)
            })
            .collect();

        if scale <= 0.0 {
            scale = 0.0001;
        }

        let mut max_noise_height = f64::MIN;
        let mut min_noise_height = f64::MAX;

        let mut noise_map = vec![0.0; (width * height) as usize];
        for y in 0..height {
            for x in 0..width {
                let mut amplitude = 1.0;
                let mut frequency = 1.0;
                let mut noise_height = 0.0;

                for i in 0..octaves {
                    let x = x as f64 / scale * frequency + octave_offsets[i as usize].x as f64;
                    let y = y as f64 / scale * frequency + octave_offsets[i as usize].y as f64;

                    let perlin_value = perlin.get([x as f64, y as f64]) * 2.0 - 1.0;
                    noise_height += perlin_value * amplitude;

                    amplitude *= persistance;
                    frequency *= lacunarity;
                }

                if noise_height > max_noise_height {
                    max_noise_height = noise_height;
                } else if noise_height < min_noise_height {
                    min_noise_height = noise_height;
                }

                noise_map[(y * width + x) as usize] = noise_height;
            }
        }

        for val in &mut noise_map {
            // normalize
            *val = (*val - min_noise_height) / (max_noise_height - min_noise_height)
        }

        NoiseMap {
            width,
            _height: height,
            noise_map,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self._height
    }

    pub fn get(&self, x: u32, y: u32) -> f64 {
        self.noise_map[(y * self.width + x) as usize]
    }
}
