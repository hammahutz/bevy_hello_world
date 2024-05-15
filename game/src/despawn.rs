use bevy::prelude::*;

use crate::{camera::GameCamera, health::Health, schedule::InGameSet, state::GameState};

const DESPAWN_DISTANCE: f32 = 80.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (despawn_far_away_entitets, despawn_dead_entities).in_set(InGameSet::DespawnEntities),
        )
        .add_systems(
            OnEnter(GameState::GameOver),
            (despawn_all_entities),
        );
    }
}

fn despawn_far_away_entitets(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), Without<GameCamera>>,
) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);

        if distance > DESPAWN_DISTANCE {
            info!("{:?} is despawn", entity);
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_dead_entities(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in query.iter() {
        if health.value <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_all_entities(mut commands: Commands, query: Query<Entity, With<Health>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
