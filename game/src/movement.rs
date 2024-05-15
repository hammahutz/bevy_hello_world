use bevy::prelude::*;

use crate::{collision_detection::Collider, schedule::InGameSet};

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_velocity, update_position)
                .chain()
                .in_set(InGameSet::EntityUpdates),
        );
    }
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}
impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}
impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub collider: Collider,
    pub model: SceneBundle,
}

fn update_position(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut position, velocity) in query.iter_mut() {
        position.translation += velocity.value * time.delta_seconds();
    }
}

fn update_velocity(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
    for (mut velocity, acceleration) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
    }
}
