use bevy::prelude::*;
const CAMERA_DISTANCE: f32 = 172.0;
pub struct CameraPlugins;

impl Plugin for CameraPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

#[derive(Component, Debug)]
pub struct GameCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(
        (Camera3dBundle {
            transform: Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0)
                .looking_at(Vec3::ZERO, Vec3::Z),
            ..default()
        },
    GameCamera),
    );
}
