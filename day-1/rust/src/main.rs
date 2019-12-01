use std::fs::File;
use std::io::Read;

fn main() {
    let input = get_input().expect("Could not open input file, does it exist?");

    let mut part1_fuel = 0;
    let mut part2_fuel = 0;

    for line in input.split_whitespace() {
        let fuel_of_module = calculate_fuel(line.parse::<f32>().unwrap());

        part1_fuel += fuel_of_module;
        part2_fuel += fuel_of_module;

        let mut last_fuel = fuel_of_module;

        loop {
            let fuel_of_fuel = calculate_fuel(last_fuel as f32);

            if fuel_of_fuel <= 0 {
                break;
            }

            part2_fuel += fuel_of_fuel;
            last_fuel = fuel_of_fuel;
        }
    }

    println!("Part 1: {}", part1_fuel);
    println!("Part 2: {}", part2_fuel);
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
