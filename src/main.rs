mod debug;
mod movement;
mod spaceship;
mod camera;
mod asteroid;
mod asset_loader;
mod collision_detection;
mod util;
mod despawn;

use bevy::prelude::*;

use collision_detection::CollisionDetectionPlugin;
use debug::DebugPlugin;
use despawn::DespawnPlugin;
use movement::MovementPlugin;
use spaceship::SpaceShipPlugin;
use camera::CameraPlugins;
use asteroid::AsteroidPlugin;
use asset_loader::AssetLoaderPlugin;

fn main() {
    App::new()
        //Bevy build-ins
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight{
            color: Color::default(),
            brightness: 750.0
        })
        .add_plugins(DefaultPlugins)
        //User defined plugins
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugins)
        .add_plugins(DebugPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(SpaceShipPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        .run();
}
