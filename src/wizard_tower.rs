use bevy::prelude::*;

use crate::map_generator::MapGenerator;

pub struct WizardTowerPlugin;

#[derive(SystemLabel, Clone, Hash, Debug, Eq, PartialEq)]
enum WizardTowerSystem {
    UpdateMapGenerator,
}

impl Plugin for WizardTowerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(MapGenerator::default())
            .add_startup_system(add_noise_map.system())
            .add_system(
                MapGenerator::update_map_generator
                    .system()
                    .label(WizardTowerSystem::UpdateMapGenerator),
            )
            .add_system(
                update_noise_map
                    .system()
                    .after(WizardTowerSystem::UpdateMapGenerator),
            );
    }
}
struct NoiseMapSprite;

fn add_noise_map(
    mut commands: Commands,
    mut generator: ResMut<MapGenerator>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = generator.validate_then_get_texture(&mut textures);

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
    for mut material in query.iter_mut() {
        let texture_handle = generator.validate_then_get_texture(&mut textures);
        *material = materials.add(texture_handle.into());
    }
}
