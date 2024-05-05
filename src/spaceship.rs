use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    collision_detection::Collider,
    movement::{Acceleration, MovingObjectBundle, Velocity},
};

const RADIUS: f32 = 10.0;
const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;

const MISSILE_RADIUS: f32 = 1.0;
const MISSILE_SPEED_SCALAR: f32 = 25.0;
const MISSILE_ACCELERATION_SCALAR: f32 = 200.0;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;

pub struct SpaceShipPlugin;

impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship).add_systems(
            Update,
            (spaceship_movement_control, spaceship_weapon_controls),
        );
    }
}

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::new(MISSILE_RADIUS),
            model: SceneBundle {
                scene: scene_assets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        Spaceship,
    ));
}

fn spaceship_movement_control(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::KeyW) {
        movement = SPACESHIP_SPEED * 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        movement = SPACESHIP_SPEED * -1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        rotation = SPACESHIP_ROTATION_SPEED * 1.0 * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        rotation = SPACESHIP_ROTATION_SPEED * -1.0 * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        roll = SPACESHIP_ROLL_SPEED * 1.0 * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        roll = SPACESHIP_ROLL_SPEED * -1.0 * time.delta_seconds();
    }

    velocity.value = -transform.forward() * movement;
    transform.rotate_y(rotation);
    transform.rotate_local_z(roll);
}

fn spaceship_weapon_controls(
    mut commands: Commands,
    query: Query<&Transform, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    let transform = query.single();

    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(-transform.forward() * MISSILE_SPEED_SCALAR),
                acceleration: Acceleration::new(-transform.forward() * MISSILE_ACCELERATION_SCALAR),
                collider: Collider::new(RADIUS),
                model: SceneBundle {
                    scene: scene_assets.missile.clone(),
                    transform: Transform::from_translation(
                        transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR,
                    ),
                    ..default()
                },
            },
            SpaceshipMissile,
        ));
    }
}
