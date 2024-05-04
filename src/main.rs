mod debug;
mod movement;
mod spaceship;
mod camera;

use bevy::prelude::*;

use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceShipPlugin;
use camera::CameraPlugins;

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
        .add_plugins(SpaceShipPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CameraPlugins)
        .add_plugins(DebugPlugin)
        .run();
}
