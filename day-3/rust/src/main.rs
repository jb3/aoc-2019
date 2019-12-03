use std::fs::File;
use std::io::Read;

mod wire;
mod parser;

use std::collections::HashSet;
use wire::{Wire, Point};

fn main() {
    let wires = get_input().expect("Could not parse wire data.");

    let sets = wires.iter().map(|wire| wire.clone().to_set()).collect::<Vec<HashSet<Point>>>();

    let a = &sets[0];
    let b = &sets[1];
    let origin = Point::new(0, 0);

    let manhattan_intersection = a.intersection(b)
        .into_iter()
        .map(|p| (p, p.manhattan_distance(&origin)))
        .collect::<Vec<(&Point, i64)>>();

    let mut smallest: (&Point, i64) = manhattan_intersection[1];

    for (p, dist) in manhattan_intersection {
        if p.x == 0 && p.y == 0 {
            continue;
        }

        if dist < smallest.1 {
            smallest = (p, dist);
        }
    }

    println!("Part 1: {}", smallest.1);

    let smallest_steps_intersection = a.intersection(b)
        .into_iter()
        .collect::<Vec<&Point>>();

    let mut first = true;
    let mut smallest: (&Point, i64) = (&origin, 0);

    for point in smallest_steps_intersection {
        if point.x == 0 && point.y == 0 {
            continue;
        }

        let mut combined = 0;

        for wire in &wires {
            combined += wire.clone().find_steps_to(point) + 1;
        }

        if first {
            smallest = (point, combined);
            first = false;
        } else {
            if combined < smallest.1 {
                smallest = (point, combined);
            }
        }
    }

    println!("Part 2: {}", smallest.1);
}

fn get_input() -> Result<Vec<Wire>, std::io::Error> {
    let mut f = File::open("../input")?;

    let mut string = String::new();

    f.read_to_string(&mut string)?;

    let wires = string.split_whitespace().map(|wire| parser::parse(wire)).collect::<Vec<Wire>>();

    Ok(wires)
}
