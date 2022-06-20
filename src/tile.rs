use std::collections::{HashMap, LinkedList};
use rand::prelude::*;
use std::str::FromStr;
use bevy::prelude::*;

const TILE_SIZE: f32 = 65.;
const SET_RADIUS: i32 = 14;

#[derive(Component)]
pub struct Tile {
    coord: (f32, f32),
    texture: &'static str,
    is_occupied: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum Connection {
    Road,
    Field,
    City,
    River,
}

impl FromStr for Connection {

    type Err = ();

    fn from_str(input: &str) -> Result<Connection, Self::Err> {
        match input {
            "Road"  => Ok(Connection::Road),
            "Field"  => Ok(Connection::Field),
            "City"  => Ok(Connection::City),
            "River"  => Ok(Connection::River),
            _      => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Type{
    pub name: &'static str,
    pub connections: Vec<Connection>,
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
    let mut tiles: HashMap<(i32, i32), Vec<Connection>> = HashMap::new();
    let mut rng = thread_rng();
    for i in -SET_RADIUS..SET_RADIUS {
        for j in -SET_RADIUS..SET_RADIUS {
            let mut val;
            let mut values_checked = LinkedList::new();
            loop {
                val = rng.gen_range(0..types.len());
                while values_checked.contains(&val) {
                    if values_checked.len() == types.len() { warn!("Impossible: {:?}, {:?}", tiles.get(&(i - 1, j)).unwrap()[1],  tiles.get(&(i, j - 1)).unwrap()[0]) }
                    val = rng.gen_range(0..types.len());
                }
                if tiles.contains_key(&(i - 1, j)) &&
                    tiles.get(&(i - 1, j)).unwrap()[1] != types[val].connections[3] {
                    values_checked.push_back(val);
                    continue;
                }
                if tiles.contains_key(&(i, j - 1)) &&
                    tiles.get(&(i, j - 1)).unwrap()[0] != types[val].connections[2] {
                    values_checked.push_back(val);
                    continue;
                }
                tiles.insert((i, j), types[val].connections.clone());
                break;
            }
            let image = asset_server.load(types[val].name);
            commands.spawn_bundle(SpriteBundle {
                texture: image.clone(),
                transform: Transform {
                    translation: Vec3::new(i as f32 * TILE_SIZE, j as f32 * TILE_SIZE, 0.),
                    scale: Vec3::new(0.51, 0.51, 0.1),
                    ..Default::default()
                },
                ..Default::default()
            })
                .insert(Tile { coord: (i as f32, j as f32), is_occupied: true });
        }
    }

}