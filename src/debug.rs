use bevy::{app::AppExit, prelude::*};



pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, exit_game);

    }
}

fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, position) in query.iter() {
        info!("Entity\t{:?} is  at position\t{:?}", entity, position);
    }
}

fn exit_game(keyboard_input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>){
    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

