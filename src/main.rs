use bevy::{prelude::*, DefaultPlugins};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use camera::CameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

mod camera;
mod player;
mod world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Ramses".into(),
                    ..default()
                }),
                ..default()
            }),
            PlayerPlugin,
            WorldPlugin,
            CameraPlugin,
            // ThirdPersonCameraPlugin,
            WorldInspectorPlugin::new(),
        ))
        .run();
}
