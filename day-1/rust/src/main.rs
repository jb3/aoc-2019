use std::fs::File;
use std::io::Read;

fn main() {
    let input = get_input().expect("Could not open input file, does it exist?");

    let mut fuel_reqs = 0;

    for line in input.split_whitespace() {
        fuel_reqs += calculate_fuel(line.parse::<f32>().unwrap());
    }

    println!("Part 1: {}", fuel_reqs);
}

fn get_input() -> Result<String, std::io::Error> {
    let mut f = File::open("../input")?;

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    Ok(buf)
}

fn calculate_fuel(mass: f32) -> i32 {
    let mut fuel = mass / 3f32;

    fuel = fuel.floor();

    fuel -= 2f32;

    fuel as i32
}
