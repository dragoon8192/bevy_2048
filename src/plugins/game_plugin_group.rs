use bevy::app::{PluginGroup, PluginGroupBuilder};

use super::{
    calculate::CalculatePlugin, game_over::GameOverPlugin, input::InputPlugin,
    main_menu::MainMenuPlugin, movement::MovementPlugin, setup::SetupPlugin, spawn::SpawnPlugin,
};

pub struct GamePluginGroup;

impl PluginGroup for GamePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SetupPlugin)
            .add(MainMenuPlugin)
            .add(SpawnPlugin)
            .add(InputPlugin)
            .add(CalculatePlugin)
            .add(MovementPlugin)
            .add(GameOverPlugin)
    }
}
