use bevy::app::{PluginGroup, PluginGroupBuilder};

use super::{
    calculate::CalculatePlugin, game_over::GameOverPlugin, input::InputPlugin,
    movement::MovementPlugin,
};

pub struct GamePluginGroup;

impl PluginGroup for GamePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(InputPlugin)
            .add(CalculatePlugin)
            .add(MovementPlugin)
            .add(GameOverPlugin)
    }
}
