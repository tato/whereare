#![deny(clippy::all)]

use bevy::prelude::*;
use wizard_tower::WizardTowerPlugin;

mod noise_map;
mod wizard_tower;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(WizardTowerPlugin)
        .run();
}
