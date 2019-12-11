#![feature(vec_remove_item)]

use std::io::Read;
use std::fs::File;

mod map;
use map::{Asteroid, Map};

fn main() {

    let mut map = generate_map();

    let los = map.calculate_line_of_sight();

    let mut los_vec: Vec<(&Asteroid, &i64)> = los.iter().collect();

    los_vec.sort_by_key(|a| a.1);

    let most_asteroids = los_vec.last().unwrap();

    println!("Part 1: {}", most_asteroids.1);

    let monitoring_station = most_asteroids.0;

    // Remove the monitoring_station

    map.remove(monitoring_station.clone());

    let shot = map.calculate_nth_shot(monitoring_station.location.clone(), 200);

    println!("Part 2: {}", shot.location.x * 100 + shot.location.y);
}

fn generate_map() -> Map {
    let mut f = File::open("../input").unwrap();

    let mut buf = String::new();

    f.read_to_string(&mut buf).unwrap();

    Map::from_input(buf)
}
