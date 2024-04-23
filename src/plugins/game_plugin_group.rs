use bevy::app::{PluginGroup, PluginGroupBuilder};

use super::{
    calculate::CalculatePlugin, game_over::GameOverPlugin, input::InputPlugin,
    movement::MovementPlugin, setup::SetupPlugin, spawn::SpawnPlugin,
};

pub struct GamePluginGroup;

impl PluginGroup for GamePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SetupPlugin)
            .add(InputPlugin)
            .add(CalculatePlugin)
            .add(MovementPlugin)
            .add(SpawnPlugin)
            .add(GameOverPlugin)
    }
}
