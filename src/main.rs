mod tile;
mod wfc;

use bevy::prelude::*;
use crate::tile::TilePlugin;
use crate::wfc::WfcPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
        .insert_resource(WindowDescriptor{
            title: "Carcassone Simulator".to_string(),
            width: 1600.,
            height: 1100.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(TilePlugin)
        .add_plugin(WfcPlugin)
        .add_startup_system(setup)
        .run();
}


fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}