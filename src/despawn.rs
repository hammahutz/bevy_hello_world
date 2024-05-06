use bevy::prelude::*;

use crate::{camera::GameCamera, schedule::InGameSet};

const DESPAWN_DISTANCE: f32 = 80.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_far_away_entitets.in_set(InGameSet::DespawnEntities));
    }
}

fn despawn_far_away_entitets(mut commands: Commands, query: Query<(Entity, &GlobalTransform), Without<GameCamera>>) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);

        if distance > DESPAWN_DISTANCE {
            info!("{:?} is despawn", entity);
            commands.entity(entity).despawn_recursive();
        }
    }

}