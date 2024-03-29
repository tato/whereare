use bevy::prelude::*;
use noise::{NoiseFn, OpenSimplex, Seedable};
use rand::{Rng, SeedableRng};

pub fn generate(
    width: u32,
    height: u32,
    seed: u32,
    mut scale: f64,
    octaves: u32,
    persistance: f64,
    lacunarity: f64,
    offset: Vec2,
) -> Vec<f64> {
    let simplex = OpenSimplex::new().set_seed(seed);
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

    let half_width = width as f64 / 2.0;
    let half_height = height as f64 / 2.0;

    let mut max_noise_height = f64::MIN;
    let mut min_noise_height = f64::MAX;

    let mut noise_map = vec![0.0; (width * height) as usize];
    for y in 0..i64::from(height) {
        for x in 0..i64::from(width) {
            let mut amplitude = 1.0;
            let mut frequency = 1.0;
            let mut noise_height = 0.0;

            for i in 0..octaves {
                let x = (x as f64 - half_width) / scale * frequency
                    + octave_offsets[i as usize].x as f64;
                let y = (y as f64 - half_height) / scale * frequency
                    + octave_offsets[i as usize].y as f64;

                let simplex_value = simplex.get([x as f64, y as f64]) * 2.0 - 1.0;
                noise_height += simplex_value * amplitude;

                amplitude *= persistance;
                frequency *= lacunarity;
            }

            if noise_height > max_noise_height {
                max_noise_height = noise_height;
            } else if noise_height < min_noise_height {
                min_noise_height = noise_height;
            }

            noise_map[(y * i64::from(width) + x) as usize] = noise_height;
        }
    }

    for val in &mut noise_map {
        // normalize
        *val = (*val - min_noise_height) / (max_noise_height - min_noise_height);
        // *val = 1.0 - *val;
    }

    noise_map
}
