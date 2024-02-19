use bevy::prelude::*;

const WINDOW_SIZE: f32 = 500.0;

fn main() {
    let window = Window {
        title: "2048".to_string(),
        resolution: (WINDOW_SIZE, WINDOW_SIZE).into(),
        ..default()
    };
    let primary_window = Some(window);

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window,
            ..default()
        }))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
