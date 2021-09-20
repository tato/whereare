#![deny(clippy::all)]

use bevy::prelude::*;
use wizard_tower::WizardTowerPlugin;

/// Generate noise maps
mod noise_map;
/// Global state
mod wizard_tower;

// struct Person;

// struct Name(String);

// fn add_people(mut commands: Commands) {
//     commands
//         .spawn()
//         .insert(Person)
//         .insert(Name("Elaina Proctor".to_string()));
//     commands
//         .spawn()
//         .insert(Person)
//         .insert(Name("Renzo Hume".to_string()));
//     commands
//         .spawn()
//         .insert(Person)
//         .insert(Name("Zayna Nieves".to_string()));
// }

// struct GreetTimer(Timer);

// fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
//     if timer.0.tick(time.delta()).just_finished() {
//         for name in query.iter() {
//             println!("Hello {}!", name.0);
//         }
//     }
// }

// pub struct HelloPlugin;

// impl Plugin for HelloPlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         app
//             .insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
//             .add_startup_system(add_people.system())
//             .add_system(greet_people.system());
//     }
// }

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(WizardTowerPlugin)
        .run();
}
