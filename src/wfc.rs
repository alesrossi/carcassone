use std::collections::linked_list::LinkedList;
use bevy::prelude::*;
use rand::{Rng, thread_rng};
use crate::tile::Type;

const TILE_SIZE: f32 = 65.;
const SET_RADIUS: i32 = 14;

pub type Wfc = Vec<Vec<LinkedList<Type>>>;

pub struct WfcPlugin;

impl Plugin for WfcPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(initialise)
            .add_system(main_loop);
    }
}

fn initialise(
    types: Res<LinkedList<Type>>,
) -> Wfc {
    let mut wave_function: Wfc = Vec::new();
    for _ in -SET_RADIUS..SET_RADIUS {
        let mut line: Vec<LinkedList<Type>> = Vec::new();
        for _ in -SET_RADIUS..SET_RADIUS {
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
    dbg!("{}", res);
    res
}

fn iterate(mut wfc: ResMut<Wfc>) {
    let mut min:(usize, usize) = (0, 0);
    wfc.iter().enumerate()
        .for_each(|line| line.1.iter().enumerate()
            .for_each(|elem| if elem.1.len() < wfc[min.0][min.1].len() && elem.1.len() != 1 { min = (line.0, elem.0); }));

    wfc[min.0][min.1] = random_node(&wfc[min.0][min.1]);

    let mut stack: LinkedList<(usize, usize)> = LinkedList::new();
    stack.push_front(min);
    while !stack.is_empty() {
        let current = stack.pop_front().unwrap();

        for (i, valid_neighbour) in valid_neighbours(current, &wfc).into_iter() {
                let res: LinkedList<Type> = match i {
                    0 => wfc[valid_neighbour.0][valid_neighbour.1]
                        .clone()
                        .into_iter()
                        .filter(|elem| elem.connections[2] == wfc[current.0][current.1].pop_front().unwrap().connections[0])
                        .collect::<LinkedList<Type>>(),
                    1 => wfc[valid_neighbour.0][valid_neighbour.1]
                        .clone()
                        .into_iter()
                        .filter(|elem| elem.connections[3] == wfc[current.0][current.1].pop_front().unwrap().connections[1])
                        .collect::<LinkedList<Type>>(),
                    2 => wfc[valid_neighbour.0][valid_neighbour.1]
                        .clone()
                        .into_iter()
                        .filter(|elem| elem.connections[0] == wfc[current.0][current.1].pop_front().unwrap().connections[2])
                        .collect::<LinkedList<Type>>(),
                    3 => wfc[valid_neighbour.0][valid_neighbour.1]
                        .clone()
                        .into_iter()
                        .filter(|elem| elem.connections[1] == wfc[current.0][current.1].pop_front().unwrap().connections[3])
                        .collect::<LinkedList<Type>>(),
                    _ => continue,
                };
                wfc[current.0][current.1] = res;
        }
    }

}

fn valid_neighbours(coords: (usize, usize), wfc :&Wfc) -> Vec<(usize, (usize, usize))>{
    let mut res = Vec::new();
    if wfc.len() < coords.0 + 1 { res.push((0, (coords.0 + 1, coords.1))) }
    if wfc.len() < coords.1 + 1 { res.push((1, (coords.0, coords.1 + 1))) }
    if 0 < coords.0 { res.push((2, (coords.0 - 1, coords.1))) }
    if 0 < coords.1 { res.push((3, (coords.0, coords.1 - 1))) }
    res
}

fn random_node(v: &LinkedList<Type>) -> LinkedList<Type> {
    let mut rng = thread_rng();
    let index = rng.gen_range(0..v.len());
    let mut e = Type { name: "", connections: vec![] };
    for (i, elem) in v.iter().enumerate() {
        if i != index { continue; }
        e = elem.clone();
        break;
    }
    LinkedList::from([e])
}


