use bevy::app::*;
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::plugin::EntropyPlugin;

mod bundle;
mod components;
mod constants;
mod error;
mod plugins;
mod resources;
mod states;
mod structs;
mod util;

use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use plugins::game_plugin_group::GamePluginGroup;

fn main() {
    let window = Window {
        title: "2048".to_string(),
        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
        ..default()
    };

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(window),
            ..default()
        }))
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(GamePluginGroup)
        .run();
}
