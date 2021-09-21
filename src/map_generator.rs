
use bevy::{prelude::*, render::texture::{SamplerDescriptor}};
use bevy_inspector_egui::{Inspectable};

use crate::noise_map;

#[derive(Inspectable)]
pub struct MapGenerator {
    map_size: u32,
    seed: String,
    noise_scale: f64,
    octaves: u32,
    persistance: f64,
    lacunarity: f64,
    offset: Vec2,
    draw_mode: DrawMode,
    regions: Vec<TerrainType>,
}

impl Default for MapGenerator {
    fn default() -> Self {
        Self {
            map_size: 128,
            seed: "wizard tower".to_string(),
            noise_scale: 27.0,
            octaves: 4,
            persistance: 0.5,
            lacunarity: 2.0,
            offset: Vec2::ZERO,
            regions: vec![
                TerrainType {
                    name: "Water".into(),
                    height: 0.1,
                    color: Color::BLUE,
                },
                TerrainType {
                    name: "Sand".into(),
                    height: 0.15,
                    color: Color::YELLOW,
                },
                TerrainType {
                    name: "Grass".into(),
                    height: 0.60,
                    color: Color::GREEN,
                },
                TerrainType {
                    name: "Mountain".into(),
                    height: 0.95,
                    color: Color::MAROON,
                },
                TerrainType {
                    name: "Peak".into(),
                    height: 1.00,
                    color: Color::WHITE,
                },
            ],
            draw_mode: DrawMode::NoiseMap,
        }
    }
}

impl MapGenerator {
    pub fn validate_then_get_texture(&mut self) -> Texture {
        let generator = self;

        generator.map_size = generator.map_size.max(1);
        generator.lacunarity = generator.lacunarity.max(1.0);
        generator.persistance = generator.persistance.max(0.0).min(1.0);
    
        let mut noise_map = noise_map::generate(
            generator.map_size,
            generator.map_size,
            calculate_u32_seed_from_str(&generator.seed),
            generator.noise_scale,
            generator.octaves,
            generator.persistance,
            generator.lacunarity,
            generator.offset,
        );

        let width = generator.map_size;
        let height = generator.map_size;

        let radius = width.min(height) as f64 / 2.0;
        let dmz_radius = radius * 1.0 / 4.0;

        let center = UVec2::new(width / 2, height / 2).as_f64();
        for y in 0..height {
            for x in 0..width {
                let point = UVec2::new(x, y).as_f64();
                let distance = (center - point).length();

                let the_difference = if distance >= radius {
                    1.0
                } else if distance <= dmz_radius {
                    0.0
                } else {
                    (distance - dmz_radius) / (radius - dmz_radius)
                };
                let the_difference = 1.0 - the_difference;

                let i = (y * width + x) as usize;
                noise_map[i] = noise_map[i] * the_difference;
            }
        }

        match generator.draw_mode {
            DrawMode::NoiseMap => get_texture_for_height_map(&noise_map, width, height),
            DrawMode::ColorMap => {
                let noise_map_colors: Vec<_> = noise_map.iter().map(|current_height| {
                    for region in &generator.regions {
                        if *current_height < region.height {
                            return region.color;
                        }
                    }
                    Color::BLACK
                }).collect();

                get_texture_for_color_map(&noise_map_colors, width, height) 
            }
        }
    }
}

fn get_texture_for_height_map(height_map: &[f64], width: u32, height: u32) -> Texture {
    let height_map_colors: Vec<_> = height_map.iter().map(|height| {
        let height = *height as f32;
        Color::rgba(height, height, height, 1.0)
    }).collect();

    get_texture_for_color_map(&height_map_colors, width, height)
}

fn get_texture_for_color_map(color_map: &[Color], width: u32, height: u32) -> Texture {
    use bevy::render::texture::{Extent3d, TextureDimension, TextureFormat};
    
    let colors_capacity = (width * height * 4) as usize;
    let mut colors = Vec::with_capacity(colors_capacity);

    for y in 0..height {
        for x in 0..width {
            let color = color_map[(y * width + x) as usize];
            colors.push((color.r() * 255.0) as u8);
            colors.push((color.g() * 255.0) as u8);
            colors.push((color.b() * 255.0) as u8);
            colors.push((color.a() * 255.0) as u8);
        }
    }

    let mut texture = Texture::new(
        Extent3d::new(width as u32, height as u32, 1),
        TextureDimension::D2,
        colors,
        TextureFormat::Rgba8UnormSrgb,
    );
    texture.sampler = SamplerDescriptor {
        address_mode_u: bevy::render::texture::AddressMode::ClampToEdge,
        address_mode_v: bevy::render::texture::AddressMode::ClampToEdge,
        address_mode_w: bevy::render::texture::AddressMode::ClampToEdge,
        mag_filter: bevy::render::texture::FilterMode::Nearest,
        ..Default::default()
    };

    texture
}

#[derive(Inspectable)]
struct TerrainType {
    name: String,
    height: f64,
    color: Color, 
}

impl Default for TerrainType {
    fn default() -> Self {
        Self { name: Default::default(), height: Default::default(), color: Default::default() }
    }
}


#[derive(Inspectable)]
enum DrawMode {
    NoiseMap, ColorMap
}

impl Default for DrawMode {
    fn default() -> Self {
        DrawMode::ColorMap
    }
}

fn calculate_u32_seed_from_str(seed: &str) -> u32 {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };
    
    let mut s = DefaultHasher::new();
    seed.hash(&mut s);
    (s.finish() % u64::from(u32::MAX)) as u32
}
