use rand::prelude::*;
use std::str::FromStr;
use bevy::prelude::*;

const TILE_SPRITE: &str = "tile.png";
const MAPPINGS: &str = "mappings.txt";
const TILE_SIZE: f32 = 65.;
const SET_RADIUS: i32 = 100;

#[derive(Component)]
pub struct Tile {
    coord: (f32, f32),
    is_occupied: bool,
}

#[derive(Debug)]
enum Connection {
    Road,
    Field,
    City,
}

impl FromStr for Connection {

    type Err = ();

    fn from_str(input: &str) -> Result<Connection, Self::Err> {
        match input {
            "Road"  => Ok(Connection::Road),
            "Field"  => Ok(Connection::Field),
            "City"  => Ok(Connection::City),
            _      => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Type{
    name: &'static str,
    connections: Vec<Connection>,
}



pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app
            // Structs and enums can all be used as resources
            .insert_resource(set_types())
            .add_startup_system(spawn_tiles);
    }
}

fn set_types(
) ->  Vec<Type> {
    let mut types: Vec<Type> = Vec::new();
    for line in include_str!("mappings.txt").lines() {
        let mut split = line.split_whitespace();
        let name = split.next().unwrap();
        let cons = split
            .next()
            .unwrap()
            .split(':')
            .map(|con| Connection::from_str(con).unwrap())
            .collect::<Vec<Connection>>();
        types.push(Type {name, connections: cons });
    }
    types
}

fn spawn_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    types: Res<Vec<Type>>
){
    let mut rng = thread_rng();
    for i in -SET_RADIUS..SET_RADIUS {
        for j in -SET_RADIUS..SET_RADIUS {
            let rng = rng.gen_range(0..16);
            let image = asset_server.load(types[rng].name);
            commands.spawn_bundle(SpriteBundle {
                texture: image.clone(),
                transform: Transform {
                    translation: Vec3::new(i as f32 * TILE_SIZE, j as f32 * TILE_SIZE, 0.),
                    scale: Vec3::new(0.53, 0.53, 0.1),
                    ..Default::default()
                },
                ..Default::default()
            })
                .insert(Tile { coord: (i as f32, j as f32), is_occupied: true });
        }
    }

}