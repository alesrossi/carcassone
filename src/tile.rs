use bevy::prelude::*;

const TILE_SPRITE: &str = "tile.png";

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        debug!("inside");
        app
            .add_startup_system(spawn_tile);
    }
}

fn spawn_tile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    let image = asset_server.load(TILE_SPRITE);
    commands.spawn_bundle(SpriteBundle {
        texture: image,
        transform: Transform {
            translation: Vec3::new(25., 25., 0.),
            scale: Vec3::new(0.5, 0.5, 0.1),
            ..Default::default()
        },
        ..Default::default()
    });

}