use bevy::prelude::*;

fn load_assets(asset_server: Res<AssetServer>) {
    let handle: Handle<Asset> = asset_server.load("mono.png");
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app.add_systems(Startup, load_assets);
    }
}