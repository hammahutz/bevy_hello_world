use bevy::prelude::*;
pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub asteroid: Handle<Scene>,
    pub missile: Handle<Scene>,
    pub spaceship: Handle<Scene>,
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        asteroid: asset_server.load("rock.glb#Scene0"),
        spaceship: asset_server.load("Spaceship.glb#Scene0"),
        missile: asset_server.load("Pickup Jar.glb#Scene0"),
    }
}
