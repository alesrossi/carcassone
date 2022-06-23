use std::collections::HashMap;
use std::collections::linked_list::LinkedList;
use std::str::FromStr;
use bevy::prelude::*;
use crate::wfc::Wfc;





#[derive(Debug, Clone, PartialEq)]
pub enum Connection {
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
            .insert_resource(set_types());
    }
}

fn set_types(
) ->  LinkedList<Type> {
    let mut types: LinkedList<Type> = LinkedList::new();
    for line in include_str!("mappings.txt").lines() {
        let mut split = line.split_whitespace();
        let name = split.next().unwrap();
        let cons = split
            .next()
            .unwrap()
            .split(':')
            .map(|con| Connection::from_str(con).unwrap())
            .collect::<Vec<Connection>>();
        types.push_back(Type {name, connections: cons });
    }
    types
}



fn spawn_tiles(
    wfc: ResMut<Wfc>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
){
    let _: HashMap<(i32, i32), Vec<Connection>> = HashMap::new();
    // let mut rng = thread_rng();
    // wfc.iter()
    //     .for_each(|line| line.iter()
    //         .for_each(|elem|
    //                   {
    //                       let image = asset_server.load(elem.pop_front().unwrap());
    //                       commands.spawn_bundle(SpriteBundle {
    //                           texture: image.clone(),
    //                           transform: Transform {
    //                               translation: Vec3::new(i as f32 * TILE_SIZE, j as f32 * TILE_SIZE, 0.),
    //                               scale: Vec3::new(0.51, 0.51, 0.1),
    //                               ..Default::default()
    //                           },
    //                           ..Default::default()
    //                       });
    //                   }
    //         ));


}