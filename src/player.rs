use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::dynamics::{GravityScale, RigidBody};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, config_player))
            .add_systems(Update, (player_movement, player_animations));
    }
}

#[derive(Debug)]
enum EntityState {
    Walking,
    Idle,
}

#[derive(Resource)]
struct LogTiming {
    timer: Timer,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct Destination(Vec3);

#[derive(Component)]
struct PlayerState {
    state: EntityState,
}

#[derive(Resource)]
struct PlayerAnimations(Vec<Handle<AnimationClip>>);

fn config_player(mut commands: Commands) {
    commands.insert_resource(LogTiming {
        timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
    })
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PlayerAnimations(vec![
        asset_server.load("human.glb#Animation0"),
        asset_server.load("human.glb#Animation1"),
    ]));

    let player = (
        SceneBundle {
            scene: asset_server.load("human.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 2.0, 0.0), //.with_scale(Vec3::new(0.1, 0.1, 0.1)),
            ..default()
        },
        Player,
        Speed(2.5),
        Destination(Vec3::ZERO),
        PlayerState {
            state: EntityState::Idle,
        },
        Name::new("Player"),
    );

    // commands.spawn(palyer);
    commands
        .spawn(RigidBody::Dynamic)
        .insert(player)
        .insert(GravityScale(0.5));
}

fn player_movement(
    buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
    mut player_query: Query<
        (&mut Transform, &Speed, &mut Destination, &mut PlayerState),
        With<Player>,
    >,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut log_timing: ResMut<LogTiming>,
) {
    log_timing.timer.tick(time.delta());

    for (mut player_transform, player_speed, mut player_destination, mut player_state) in
        player_query.iter_mut()
    {
        if buttons.just_pressed(MouseButton::Left) {
            // Left button was pressed
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
            // player_destination.0 = player_transform.translation + ray.get_point(distance);
            player_destination.0 = ray.get_point(distance);
            player_transform.look_at(player_destination.0, Vec3::Y);
        }

        let direction = player_destination.0 - player_transform.translation;

        // se sono distante dalla destinazione
        if direction.length() > 0.1 {
            player_state.state = EntityState::Walking;
            // Normalizza la direzione per ottenere un vettore unitario
            let normalized_direction = direction.normalize();
            // Calcola la quantità di spostamento sulla direzione normalizzata
            let movement = normalized_direction * player_speed.0 * time.delta_seconds();

            player_transform.translation += movement;
        } else {
            player_state.state = EntityState::Idle;
        }

        if log_timing.timer.finished() {
            // debug!(
            //     "Player {} - Dest {} - State {:?}",
            //     player_transform.translation, player_destination.0, player_state.state
            // );
        }
    }
}

fn player_animations(
    animations: Res<PlayerAnimations>,
    player_query: Query<&mut PlayerState, With<Player>>,
    mut player_animations: Query<&mut AnimationPlayer>,

    mut current_animation: Local<usize>,
) {
    for player in player_query.iter() {
        match player.state {
            EntityState::Walking => {
                *current_animation = 1;
            }
            EntityState::Idle => {
                *current_animation = 0;
            }
        }
        for mut player_animation in &mut player_animations {
            player_animation
                .play_with_transition(
                    animations.0[*current_animation].clone_weak(),
                    Duration::from_millis(200),
                )
                .repeat();
        }
    }
}
