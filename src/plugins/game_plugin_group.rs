use bevy::app::{PluginGroup, PluginGroupBuilder};

use super::calculate::CalculatePlugin;
use super::input::InputPlugin;

struct GamePluginGroup;

impl PluginGroup for GamePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(InputPlugin)
            .add(CalculatePlugin)
    }
}
