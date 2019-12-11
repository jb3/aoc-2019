use std::io::Read;
use std::fs::File;

mod map;
use map::{Asteroid, Map};

fn main() {

    let mut map = generate_map();

    let los = map.calculate_line_of_sight();

    let mut los_vec: Vec<(&Asteroid, &i64)> = los.iter().collect();

    los_vec.sort_by_key(|a| a.1);

    println!("Part 1: {}", los_vec.last().unwrap().1);
}

fn generate_map() -> Map {
    let mut f = File::open("../input").unwrap();

    let mut buf = String::new();

    f.read_to_string(&mut buf).unwrap();

    Map::from_input(buf)
}
