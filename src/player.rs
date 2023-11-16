use std::time::Duration;

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement, player_animations));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct Speed(f32);

#[derive(Resource)]
struct PlayerAnimations(Vec<Handle<AnimationClip>>);

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PlayerAnimations(vec![
        asset_server.load("human.glb#Animation0"),
        asset_server.load("human.glb#Animation1"),
    ]));
    let palyer = (
        SceneBundle {
            scene: asset_server.load("human.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0), //.with_scale(Vec3::new(0.1, 0.1, 0.1)),
            ..default()
        },
        Player,
        Speed(2.5),
        Name::new("Player"),
    );

    commands.spawn(palyer);
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Speed), With<Player>>,
    camera_3db_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    // calcolo il punto in coordinate world in cui si trova il mouse
    let (camera, camera_transform) = camera_query.single();
    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };
    let Some(distance) = ray.intersect_plane(Vec3::ZERO, Vec3::Y) else {
        return;
    };
    let point = ray.get_point(distance);

    for (mut player_transform, player_speed) in player_query.iter_mut() {
        let mut direction: Vec3 = Vec3::ZERO;

        // calcolo il punto del mouse rispetto alla posizione del player
        let relative_point = point - player_transform.translation;

        player_transform.look_to(-relative_point, Vec3::Y);
        debug!(
            "Player {} - Pointer {}",
            player_transform.translation, point
        );

        if keys.pressed(KeyCode::W) {
            direction += relative_point; //camera_3db.forward();
        }

        direction.y = 0.0;
        let movement: Vec3 = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
        player_transform.translation += movement;

        // }
    }
}

fn player_animations(
    keyboard_input: Res<Input<KeyCode>>,
    animations: Res<PlayerAnimations>,
    mut players: Query<&mut AnimationPlayer>,
    mut current_animation: Local<usize>,
) {
    for mut player in &mut players {
        if keyboard_input.any_pressed([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D]) {
            *current_animation = 1;
        } else {
            *current_animation = 0;
        }

        player
            .play_with_transition(
                animations.0[*current_animation].clone_weak(),
                Duration::from_millis(200),
            )
            .repeat();
    }
}
