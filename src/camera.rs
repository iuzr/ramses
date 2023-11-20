use crate::player::Player;
use bevy::prelude::*;
pub struct CameraPlugin;

const CAMERA_Z_OFFSET: f32 = 14.0;
const CAMERA_Y_OFFSET: f32 = 6.0;
const CAMERA_X_OFFSET: f32 = 0.0;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        //app.add_systems(Update, camera_movements);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = (Camera3dBundle {
        transform: Transform::from_xyz(CAMERA_X_OFFSET, CAMERA_Y_OFFSET, CAMERA_Z_OFFSET)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },);

    commands.spawn(camera);
}

// Segue il player
fn camera_movements(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    let Ok(player_query) = player_query.get_single() else {
        return;
    };
    let mut cam = match camera_query.get_single_mut() {
        Ok(c) => c,
        Err(e) => Err(format!("Error retrieving camera {}", e)).unwrap(),
    };

    cam.translation =
        player_query.translation + Vec3::Z * CAMERA_Z_OFFSET + Vec3::Y * CAMERA_Y_OFFSET;
}
