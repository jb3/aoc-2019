use std::fs::File;
use std::io::Read;

use itertools::Itertools;

mod interpreter;

fn main() {
    let input = get_input().expect("Could not open input, does the file exist?");

    let codes = input.split_terminator(",")
        .map(|x| x.trim())
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut biggest = 0;

    for comb in (0..=4).into_iter().permutations(5) {
        let signal = run_combination_part_1(comb, &codes);

        if signal > biggest {
            biggest = signal;
        }
    }

    println!("Part 1: {}", biggest);

    biggest = 0;

    for comb in (0..=9).into_iter().permutations(5) {
        let signal = run_combination_part_2(comb, &codes);

        if signal > biggest {
            biggest = signal;
        }
    }

    println!("Part 2: {}", biggest);
}

fn run_combination_part_1(settings: Vec<i64>, code: &Vec<i64>) -> i64 {
    let mut last_output = 0;

    for setting in settings {
        let mut software = interpreter::Interpreter::new(code.clone(), vec![setting, last_output]);

        while software.is_running {
            software.step();
        }

        last_output = software.last_output;
    };

    last_output
}

fn run_combination_part_2(settings: Vec<i64>, code: &Vec<i64>) -> i64 {
    let mut last_output = 0;

    let mut amplifiers: Vec<interpreter::Interpreter> = vec![];

    for setting in settings {
        let amp = interpreter::Interpreter::new(code.clone(), vec![setting]);
        amplifiers.push(amp);
    }

    let mut index = 0;

    'outer: loop {
        let amplifier = &mut amplifiers[index % 5];

        amplifier.add_input(last_output);

        while !amplifier.has_outputted {
            amplifier.step();
            if !amplifier.is_running {
                break 'outer;
            }
        }

        amplifier.has_outputted = false;

        last_output = amplifier.last_output;

        index += 1;
    }

    amplifiers.last().unwrap().last_output
}


fn get_input() -> Result<String, std::io::Error> {
    let mut f = File::open("../input")?;

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    Ok(buf)
}
