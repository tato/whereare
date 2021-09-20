use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use bevy::{prelude::*, render::texture::SamplerDescriptor};
use bevy_inspector_egui::{Inspectable, InspectorPlugin};

use crate::noise_map::NoiseMap;

pub struct WizardTowerPlugin;

impl Plugin for WizardTowerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(MapGenerator::default())
            .add_plugin(InspectorPlugin::<MapGenerator>::new())
            .add_startup_system(add_noise_map.system())
            .add_system(update_noise_map.system());
    }
}

#[derive(Inspectable)]
struct MapGenerator {
    map_size: u32,
    seed: String,
    noise_scale: f64,
    octaves: u32,
    persistance: f64,
    lacunarity: f64,
    offset: Vec2,
}

impl Default for MapGenerator {
    fn default() -> Self {
        Self {
            map_size: 100,
            seed: "wizard tower".to_string(),
            noise_scale: 27.0,
            octaves: 4,
            persistance: 0.5,
            lacunarity: 2.0,
            offset: Vec2::ZERO,
        }
    }
}

struct NoiseMapSprite;

fn get_texture_from_map_generator(generator: &MapGenerator) -> Texture {
    let noise_map = NoiseMap::generate(
        generator.map_size,
        generator.map_size,
        calculate_u32_seed_from_str(&generator.seed),
        generator.noise_scale,
        generator.octaves,
        generator.persistance,
        generator.lacunarity,
        generator.offset,
    );

    use bevy::render::texture::{Extent3d, TextureDimension, TextureFormat};

    let noise_map_colors_capacity = (noise_map.width() * noise_map.height() * 4) as usize;
    let mut noise_map_colors = Vec::with_capacity(noise_map_colors_capacity);
    for y in 0..noise_map.height() {
        for x in 0..noise_map.width() {
            let f = (noise_map.get(x, y) * 255.0) as u8;
            noise_map_colors.push(f);
            noise_map_colors.push(f);
            noise_map_colors.push(f);
            noise_map_colors.push(0xff);
        }
    }

    let (bytes, width, height) = (noise_map_colors, noise_map.width(), noise_map.height());

    let mut texture = Texture::new(
        Extent3d::new(width as u32, height as u32, 1),
        TextureDimension::D2,
        bytes,
        TextureFormat::Rgba8Unorm,
    );
    texture.sampler = SamplerDescriptor {
        mag_filter: bevy::render::texture::FilterMode::Linear,
        ..Default::default()
    };
    
    texture
}

fn add_noise_map(
    mut commands: Commands,
    generator: Res<MapGenerator>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture = get_texture_from_map_generator(&generator);

    let texture_handle = textures.add(texture);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn()
        .insert(NoiseMapSprite)
        .insert_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            sprite: Sprite {
                size: Vec2::new(512.0, 512.0),
                resize_mode: SpriteResizeMode::Manual,
                ..Default::default()
            },
            ..Default::default()
        });
}

fn update_noise_map(generator: Res<MapGenerator>, mut query: Query<&mut Handle<ColorMaterial>, With<NoiseMapSprite>>, 
mut textures: ResMut<Assets<Texture>>,
mut materials: ResMut<Assets<ColorMaterial>>,) {
    if generator.is_changed() {
        for mut material in query.iter_mut() {
            let texture = get_texture_from_map_generator(&generator);
            let texture_handle = textures.add(texture);
            *material = materials.add(texture_handle.into());
        }
    }
}

fn calculate_u32_seed_from_str(seed: &str) -> u32 {
    let mut s = DefaultHasher::new();
    seed.hash(&mut s);
    (s.finish() % u64::from(u32::MAX)) as u32
}
