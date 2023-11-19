use bevy::prelude::*;
use std::f32::consts::PI;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_map, spawn_light));
    }
}

fn spawn_light(mut commands: Commands) {
    let sun = (
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 10.0, 0.0),
                rotation: Quat::from_rotation_x(-PI / 4.),
                ..default()
            }
            .into(),
            ..default()
        },
        Name::new("Sun"),
    );

    commands.spawn(sun);
}

fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(50.0))),
            material: materials.add(Color::DARK_GREEN.into()),
            ..default()
        },
        Name::new("Map"),
    );

    commands.spawn(floor);
}
