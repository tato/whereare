use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use bevy::prelude::*;
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

    let texture = Texture::new(
        Extent3d::new(width as u32, height as u32, 1),
        TextureDimension::D2,
        bytes,
        TextureFormat::Rgba8Unorm,
    );
    
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
            ..Default::default()
        });
}

fn update_noise_map(generator: Res<MapGenerator>, mut query: Query<(&NoiseMapSprite, &mut Handle<ColorMaterial>)>, 
mut textures: ResMut<Assets<Texture>>,
mut materials: ResMut<Assets<ColorMaterial>>,) {
    if generator.is_changed() {
        for (_, mut material) in query.iter_mut() {
            let texture = get_texture_from_map_generator(&generator);
            let texture_handle = textures.add(texture);
            *material = materials.add(texture_handle.into());
        }
    }
}

pub struct World {
    // map_size: u32,
// seed: imgui::ImString,
// noise_scale: f64,

// octaves: u32,
// persistance: f64,
// lacunarity: f64,
// offset: Vec2,

// map: WorldMap,
}

impl World {
    // pub const WINDOW_WIDTH: u32 = 1800;
    // pub const WINDOW_HEIGHT: u32 = Self::WINDOW_WIDTH / 16 * 9;

    pub fn new() -> Self {
        // let map_size = 100;
        // let seed = imgui::ImString::new("supok");
        // let noise_scale = map_size as f64 / 4.0;
        // let octaves = 4;
        // let persistance = 0.5;
        // let lacunarity = 2.0;
        // let offset = Vec2::ZERO;

        // let seed_u64 = calculate_u32_seed_from_str(seed.to_str());

        Self {
            // map_size,
            // seed,
            // noise_scale,
            // octaves,
            // persistance,
            // lacunarity,
            // offset,
            // map: WorldMap::generate(
            //     map_size,
            //     map_size,
            //     seed_u64,
            //     noise_scale,
            //     octaves,
            //     persistance,
            //     lacunarity,
            //     offset,
            // ),
        }
    }

    pub fn update(&mut self) {}

    // pub fn update_on_gui(&mut self, ui: &imgui::Ui) {
    //     imgui::Window::new(im_str!("World"))
    //         .size([300.0, 110.0], imgui::Condition::FirstUseEver)
    //         .build(ui, || {
    //             let mut updated = false;

    //             updated = updated
    //                 || imgui::Slider::new(im_str!("Map Size"))
    //                     .range(10..=1000)
    //                     .build(ui, &mut self.map_size);

    //             updated = updated
    //                 || ui.input_text(im_str!("Seed"), &mut self.seed).build();

    //             updated = updated
    //                 || imgui::Slider::new(im_str!("Noise Scale"))
    //                     .range(0.0..=100.0)
    //                     .build(ui, &mut self.noise_scale);

    //             updated = updated
    //                 || imgui::Slider::new(im_str!("Octaves"))
    //                     .range(0..=10)
    //                     .build(ui, &mut self.octaves);

    //             updated = updated
    //                 || imgui::Slider::new(im_str!("Persistance"))
    //                     .range(0.0..=1.0)
    //                     .build(ui, &mut self.persistance);

    //             updated = updated
    //                 || imgui::Slider::new(im_str!("Lacunarity"))
    //                     .range(1.0..=100.0)
    //                     .build(ui, &mut self.lacunarity);

    //             if updated {
    //                 self.map = WorldMap::generate(
    //                     self.map_size,
    //                     self.map_size,
    //                     calculate_u32_seed_from_str(self.seed.to_str()),
    //                     self.noise_scale,
    //                     self.octaves,
    //                     self.persistance,
    //                     self.lacunarity,
    //                     self.offset,
    //                 );
    //             }
    //         });
    // }

    // pub fn draw(&self, frame: &mut [u8]) {
    //     let map_size_on_screen = Self::WINDOW_HEIGHT / 10 * 9;

    //     let map_start_x = (Self::WINDOW_WIDTH - map_size_on_screen) / 2;
    //     let map_start_y = (Self::WINDOW_HEIGHT - map_size_on_screen) / 2;

    //     // slow
    //     // for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {

    //     // fast
    //     for i in 0..frame.len() / 4 {
    //         let x = i as u32 % Self::WINDOW_WIDTH;
    //         let y = i as u32 / Self::WINDOW_WIDTH;

    //         let inside_the_box = x >= map_start_x
    //             && x < map_start_x + map_size_on_screen
    //             && y >= map_start_y
    //             && y < map_start_y + map_size_on_screen;

    //         let rgba = if inside_the_box {
    //             let x = (((x - map_start_x) as f64 / map_size_on_screen as f64)
    //                 * self.map_size as f64) as u32;
    //             let y = (((y - map_start_y) as f64 / map_size_on_screen as f64)
    //                 * self.map_size as f64) as u32;
    //             let f = self.map.get(x, y);
    //             let f = (f * 255.0) as u8;
    //             [f, f, f, 0xff]
    //         } else {
    //             [0x48, 0xb2, 0xe8, 0xff]
    //         };

    //         // slot
    //         // pixel.copy_from_slice(&rgba);

    //         unsafe {
    //             // fast
    //             std::ptr::copy_nonoverlapping(rgba.as_ptr(), frame.as_mut_ptr().add(i * 4), 4);
    //         }
    //     }
    // }
}

fn calculate_u32_seed_from_str(seed: &str) -> u32 {
    let mut s = DefaultHasher::new();
    seed.hash(&mut s);
    (s.finish() % u64::from(u32::MAX)) as u32
}
