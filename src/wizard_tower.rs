use bevy::{prelude::*};
use bevy_inspector_egui::{InspectorPlugin};

use crate::map_generator::MapGenerator;


pub struct WizardTowerPlugin;

impl Plugin for WizardTowerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(MapGenerator::default())
            .add_plugin(InspectorPlugin::<MapGenerator>::new())
            .add_startup_system(add_noise_map.system())
            .add_system(update_noise_map.system());
    }
}
struct NoiseMapSprite;

fn add_noise_map(
    mut commands: Commands,
    mut generator: ResMut<MapGenerator>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture = generator.validate_then_get_texture();

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

fn update_noise_map(
    mut generator: ResMut<MapGenerator>,
    mut query: Query<&mut Handle<ColorMaterial>, With<NoiseMapSprite>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if generator.is_changed() {
        for mut material in query.iter_mut() {
            let texture = generator.validate_then_get_texture();
            let texture_handle = textures.add(texture);
            *material = materials.add(texture_handle.into());
        }
    }
}