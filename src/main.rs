use bevy::app::*;
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::plugin::EntropyPlugin;

mod app_state;
mod constant;
mod error;

use app_state::{
    game::plugin::game_plugin_group::GamePluginGroup, title_menu::plugin::TitleMenuPlugin,
};
use constant::layout::WINDOW;

fn main() {
    let window = Window {
        title: "2048".to_string(),
        resolution: (WINDOW.width, WINDOW.height).into(),
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
