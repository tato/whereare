use bevy::{prelude::*, render::texture::SamplerDescriptor};
use bevy_egui::{
    egui::{self, ComboBox},
    EguiContext,
};

use crate::noise_map;

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

    changed: bool,
    cached_texture: Handle<Texture>,
}

impl Default for MapGenerator {
    fn default() -> Self {
        Self {
            map_size: 256,
            seed: "wizard tower".to_string(),
            noise_scale: 50.0,
            octaves: 4,
            persistance: 0.5,
            lacunarity: 2.0,
            offset: Vec2::ZERO,
            regions: vec![
                TerrainType {
                    _name: "Water".into(),
                    height: 0.5,
                    color: Color::BLUE,
                },
                TerrainType {
                    _name: "Sand".into(),
                    height: 0.55,
                    color: Color::YELLOW,
                },
                TerrainType {
                    _name: "Grass".into(),
                    height: 0.80,
                    color: Color::GREEN,
                },
                TerrainType {
                    _name: "Mountain".into(),
                    height: 0.90,
                    color: Color::MAROON,
                },
                TerrainType {
                    _name: "Peak".into(),
                    height: 1.00,
                    color: Color::WHITE,
                },
            ],
            draw_mode: DrawMode::NoiseMap,

            changed: true,
            cached_texture: Handle::default(),
        }
    }
}

impl MapGenerator {
    fn generate_noise_map(&self) -> Vec<f64> {
        noise_map::generate(
            self.map_size,
            self.map_size,
            calculate_u32_seed_from_str(&self.seed),
            self.noise_scale,
            self.octaves,
            self.persistance,
            self.lacunarity,
            self.offset,
        )
    }

    pub fn validate_then_get_texture(&mut self, textures: &mut Assets<Texture>) -> Handle<Texture> {
        if !self.changed {
            return self.cached_texture.clone();
        }

        self.map_size = self.map_size.max(1);
        self.lacunarity = self.lacunarity.max(1.0);
        self.persistance = self.persistance.max(0.0).min(1.0);

        let noise_map = self.generate_noise_map();

        let width = self.map_size;
        let height = self.map_size;

        // let radius = width.min(height) as f64 / 2.0;
        // let dmz_radius = radius * 1.0 / 4.0;

        // let center = UVec2::new(width / 2, height / 2).as_f64();
        // for y in 0..height {
        //     for x in 0..width {
        //         let point = UVec2::new(x, y).as_f64();
        //         let distance = (center - point).length();

        //         let the_difference = if distance >= radius {
        //             1.0
        //         } else if distance <= dmz_radius {
        //             0.0
        //         } else {
        //             (distance - dmz_radius) / (radius - dmz_radius)
        //         };
        //         let the_difference = 1.0 - the_difference;

        //         let i = (y * width + x) as usize;
        //         noise_map[i] = noise_map[i] * the_difference;
        //     }
        // }

        let texture = match self.draw_mode {
            DrawMode::NoiseMap => get_texture_for_height_map(&noise_map, width, height),
            DrawMode::ColorMap => {
                let noise_map_colors: Vec<_> = noise_map
                    .iter()
                    .map(|current_height| {
                        for region in &self.regions {
                            if *current_height < region.height {
                                return region.color;
                            }
                        }
                        Color::BLACK
                    })
                    .collect();

                get_texture_for_color_map(&noise_map_colors, width, height)
            }
        };

        let texture_handle = textures.add(texture);
        self.cached_texture = texture_handle.clone();
        self.changed = false;

        texture_handle
    }

    pub fn update_map_generator(
        mut generator: ResMut<MapGenerator>,
        egui_context: ResMut<EguiContext>,
    ) {
        use egui::widgets::DragValue;

        let mut changed = false;

        egui::Window::new("Map Generator").show(egui_context.ctx(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Seed");
                changed = changed || ui.text_edit_singleline(&mut generator.seed).changed();
            });
            ui.horizontal(|ui| {
                ui.label("Offset");
                changed = changed || ui.add(DragValue::new(&mut generator.offset.x)).changed();
                changed = changed || ui.add(DragValue::new(&mut generator.offset.y)).changed();
            });
            egui::CollapsingHeader::new("Noise Map Characteristics")
                .default_open(false)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Map Size");
                        changed =
                            changed || ui.add(DragValue::new(&mut generator.map_size)).changed();
                    });
                    ui.horizontal(|ui| {
                        ui.label("Noise Scale");
                        changed =
                            changed || ui.add(DragValue::new(&mut generator.noise_scale)).changed();
                    });
                    ui.horizontal(|ui| {
                        ui.label("Octaves");
                        changed =
                            changed || ui.add(DragValue::new(&mut generator.octaves)).changed();
                    });
                    ui.horizontal(|ui| {
                        ui.label("Persistance");
                        changed =
                            changed || ui.add(DragValue::new(&mut generator.persistance)).changed();
                    });
                    ui.horizontal(|ui| {
                        ui.label("Lacunarity");
                        changed =
                            changed || ui.add(DragValue::new(&mut generator.lacunarity)).changed();
                    });
                });

            ui.horizontal(|ui| {
                ui.label("Draw Mode");
                ComboBox::from_label("Hey!")
                    .selected_text(format!("{:?}", generator.draw_mode))
                    .show_ui(ui, |ui| {
                        changed = changed || ui.selectable_value(
                            &mut generator.draw_mode,
                            DrawMode::NoiseMap,
                            "NoiseMap",
                        ).changed();
                        changed = changed || ui.selectable_value(
                            &mut generator.draw_mode,
                            DrawMode::ColorMap,
                            "ColorMap",
                        ).changed();
                    });
            });

            if ui.button("Find Shape").clicked() {
                generator.find_shape();
            }
        });

        generator.changed = changed;
    }

    fn find_shape(&mut self) {
        let noise_map = self.generate_noise_map();
        self.changed = false;
    }
}

fn get_texture_for_height_map(height_map: &[f64], width: u32, height: u32) -> Texture {
    let height_map_colors: Vec<_> = height_map
        .iter()
        .map(|height| {
            let height = *height as f32;
            Color::rgba(height, height, height, 1.0)
        })
        .collect();

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

struct TerrainType {
    _name: String,
    height: f64,
    color: Color,
}

impl Default for TerrainType {
    fn default() -> Self {
        Self {
            _name: Default::default(),
            height: Default::default(),
            color: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum DrawMode {
    NoiseMap,
    ColorMap,
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
