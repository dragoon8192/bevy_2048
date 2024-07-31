use bevy::app::*;
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::plugin::EntropyPlugin;

mod app_state;
mod bundle;
mod components;
mod constant;
mod error;
mod plugins;
mod resources;
mod state;
mod structs;
mod util;

use app_state::title_menu::plugin::TitleMenuPlugin;
use constant::layout::{WINDOW_HEIGHT, WINDOW_WIDTH};
use plugins::game_plugin_group::GamePluginGroup;

fn main() {
    let window = Window {
        title: "2048".to_string(),
        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
        ..default()
    };

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(window),
            ..default()
        }))
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(TitleMenuPlugin)
        .add_plugins(GamePluginGroup)
        .run();
}
