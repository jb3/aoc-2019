use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

mod recipe;
use recipe::*;

fn main() {
    let recipes = get_input();

    let mut leftover_chemicals: HashMap<Chemical, i64> = HashMap::new();

    let fuel_target = Chemical {
        name: "FUEL".to_string()
    };

    let ore = react(fuel_target.clone(), 1, &recipes, &mut leftover_chemicals);

    println!("Part 1: {}", ore);

    let ore_count: i64 = 1000000000000;

    let mut max_fuel = 1;

    let mut increment = 10000;

    let mut last_known_good = 1;

    loop {
        let ore = react(fuel_target.clone(), max_fuel, &recipes, &mut HashMap::new());
        let next = react(fuel_target.clone(), max_fuel + increment, &recipes, &mut HashMap::new());

        if ore < ore_count && next < ore_count {
            last_known_good = max_fuel;
            max_fuel += increment;
        } else if ore < ore_count && next > ore_count && increment != 1 {
            increment /= 10;
            max_fuel = last_known_good;
        } else if ore < ore_count && next > ore_count && increment == 1 {
            println!("Part 2: {}", max_fuel);
            break;
        }
    }
}

fn react(
    chemical: Chemical,
    amount: i64,
    reactions: &HashMap<Chemical, Recipe>,
    leftover: &mut HashMap<Chemical, i64>
) -> i64 {
    let rec = &reactions[&chemical];
    let mut ore = 0;
    let mut produced = 0;

    if let Some(&ore_rec) = rec.inputs.get(&Chemical { name: "ORE".to_string() }) {
        let repeats = (amount + rec.output.0 - 1) / rec.output.0;
        ore += ore_rec * repeats;
        produced += rec.output.0 * repeats;
    } else {
        let repeats = (amount + rec.output.0 - 1) / rec.output.0;

        for (chem, number) in &rec.inputs {
            let exist = *leftover.get(&chem).unwrap_or(&0);
            let required = number * repeats;

            if exist < required {
                ore += react(chem.clone(), required - exist, reactions, leftover);
            }

            let remaining_quant = leftover.get_mut(&chem).unwrap();
            *remaining_quant -= required;
        }
        produced += rec.output.0 * repeats;
    }

    let leftover = leftover.entry(chemical).or_insert(0);
    *leftover += produced;

    ore
}

fn get_input() -> HashMap<Chemical, Recipe> {
    let mut f = File::open("../input").unwrap();

    let mut buf = String::new();

    f.read_to_string(&mut buf).unwrap();

    buf.split_terminator("\n")
       .map(|l| Recipe::from_string(l.to_string()))
       .map(|r| (r.output.1.clone(), r.clone()))
       .collect::<HashMap<_, _>>()
}
