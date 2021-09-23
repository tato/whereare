#![deny(clippy::all)]

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use wizard_tower::WizardTowerPlugin;

mod map_generator;
mod noise_map;
mod wizard_tower;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(WizardTowerPlugin)
        // .add_startup_system(setup_egui_fonts.system())
        .run();
}

// fn setup_egui_fonts(
//     egui_context: ResMut<EguiContext>,
//     // asset_server: Res<AssetServer>,
// ) {
//     let mut fds = egui::FontDefinitions::default();
//
//     // let font_handle: Handle<Font> = asset_server.load("Cousine.ttf");
//
//     fds.font_data.insert(
//         "Cousine".to_string(),
//         std::borrow::Cow::Borrowed(include_bytes!("../assets/Cousine.ttf"))
//     );
//
//     fds.fonts_for_family.get_mut(&egui::FontFamily::Proportional).unwrap()
//         .insert(0, "Cousine".to_owned());
//
//     fds.fonts_for_family.get_mut(&egui::FontFamily::Monospace).unwrap()
//         .insert(0, "Cousine".to_owned());
//
//     egui_context.ctx().set_fonts(fds);
// }
