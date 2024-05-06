use bevy::{prelude::*, utils::HashMap};

use crate::{asteroid::Asteroid, schedule::InGameSet, spaceship::Spaceship};

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            collision_detection.in_set(InGameSet::CollisionsDetection),
        )
        .add_systems(
            Update,
            (
                handle_collisions::<Asteroid>,
                handle_collisions::<Spaceship>,
            )
                .in_set(InGameSet::DespawnEntities),
        );
    }
}

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entity: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entity: vec![],
        }
    }
}

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    for (entity_a, transform_a, collider_a) in query.iter() {
        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a != entity_b {
                let distance = transform_a
                    .translation()
                    .distance(transform_b.translation());
                if distance < collider_a.radius + collider_b.radius {
                    colliding_entities
                        .entry(entity_a)
                        .or_insert_with(Vec::new)
                        .push(entity_b);
                }
            }
        }
    }

    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entity.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider.colliding_entity.extend(collisions.iter().copied());
        }
    }
}

fn handle_collisions<T: Component>(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<T>>,
) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entity.iter() {
            //Asteroid is collided with another asteroid
            if query.get(collided_entity).is_ok() {
                continue;
            }
            //Anything else
            commands.entity(entity).despawn_recursive();
        }
    }
}
