use bevy::prelude::*;

use super::{bundle, component, constant};

pub fn create_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(constant::font::NAME);
    commands
        .spawn(bundle::Screen::default())
        .with_children(bundle::Screen::child_builder(font));
}

pub fn remove_screen(query: Query<Entity, With<component::Screen>>, mut commands: Commands) {
    commands.entity(query.single()).despawn_recursive();
}
