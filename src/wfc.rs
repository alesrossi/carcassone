use std::collections::LinkedList;
use bevy::prelude::*;
use bevy::reflect::List;
use rand::{Rng, thread_rng};
use crate::tile::Type;

const TILE_SIZE: f32 = 65.;
const SET_RADIUS: i32 = 14;

type Wfc = Vec<Vec<Vec<Type>>>;

pub struct WfcPlugin;

impl Plugin for WfcPlugin {
    fn build(&self, app: &mut App) {
        app
            // Structs and enums can all be used as resources
            .insert_resource(initialise)
            .add_system(spawn_tiles);
    }
}

fn initialise(
    types: Res<Vec<Type>>,
) -> Wfc {
    let mut wave_function: Vec<Vec<Vec<Type>>> = Vec::new();
    for x in -SET_RADIUS..SET_RADIUS {
        let mut line: Vec<Vec<Type>> = Vec::new();
        for y in -SET_RADIUS..SET_RADIUS {
            line.push(types.clone());
        }
        wave_function.push(line);
    }
    wave_function
}


fn main_loop(
    wfc: ResMut<Wfc>,
){
    if !is_collapsed(&wfc) {
        iterate(wfc);
    }
}

fn is_collapsed(wfc: &Wfc) -> bool {
    let mut res = true;
    for line in wfc {
        for elem in line {
            res = elem.len() == 1
        }
    }
    res
}

fn iterate(mut wfc: ResMut<Wfc>) {
    let mut rng = thread_rng();
    let mut min:(usize, usize) = (0, 0);
    wfc.iter().enumerate()
        .for_each(|line| line.1.iter().enumerate()
            .for_each(|elem| if elem.1.len() < wfc[min.0][min.1].len() && elem.1.len() != 1 { min = (line.0.clone(), elem.0.clone()); }));
    wfc[min.0][min.1] = vec![wfc[min.0][min.1][rng.gen_range(0..wfc[min.0][min.1].len())]];

    let mut stack: LinkedList<(usize, usize)> = LinkedList::new();
    stack.push_front(min);
    while stack.len() > 0 {
        let elem = stack.pop_front().unwrap();

        let mut count = 0;
        for (i, valid_neighbour) in valid_neighbours(elem, &wfc).iter().enumerate() {
            let val =
                wfc[valid_neighbour.0][valid_neighbour.1]
                .clone()
                .iter_mut()
                .filter(|elem| *elem.connections == wfc[elem.0][elem.1][0].connections).collect::Vec<Type>();
        }
    }

}

fn valid_neighbours(coords: (usize, usize), wfc :&Wfc) -> Vec<(usize, usize)>{
    let mut res = Vec::new();
    if wfc.len() < coords.0 + 1 { res.push((coords.0 + 1, coords.1)) }
    if wfc.len() < coords.1 + 1 { res.push((coords.0, coords.1 + 1)) }
    if 0 < coords.0 { res.push((coords.0 - 1, coords.1)) }
    if 0 < coords.1 { res.push((coords.0, coords.1 - 1)) }
    res
}
