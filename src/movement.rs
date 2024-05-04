pub struct MovementPlugin;

use bevy::prelude::*;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

fn update_position(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut position, velocity) in query.iter_mut() {
        position.translation += velocity.value * time.delta_seconds();
    }
}
