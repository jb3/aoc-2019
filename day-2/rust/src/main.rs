use std::fs::File;
use std::io::Read;

mod interpreter;

const TO_FIND: i64 = 19690720;

fn main() {
    let input = get_input().expect("Could not open input, does the file exist?");

    let mut codes = input.split_terminator(",")
        .map(|x| x.trim())
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    // Account for Error
    codes[1] = 12;
    codes[2] = 2;

    let mut interpreter = interpreter::Interpreter::new(codes);

    while interpreter.is_running {
        interpreter.step();
    }

    println!("Part 1: {}", interpreter.fetch(0));

    'outer: for i in 0..=99 {
        'inner: for j in 0..=99 {
            let mut codes = input.split_terminator(",")
                .map(|x| x.trim())
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            codes[1] = i;
            codes[2] = j;
            let mut interpreter = interpreter::Interpreter::new(codes);

            while interpreter.is_running {
                interpreter.step();
            }

            if interpreter.fetch(0) == TO_FIND {
                println!("Part 2: 100 * {} + {} = {}", i, j, 100 * i + j);
                break 'outer;
            }
        }
    }
}

fn get_input() -> Result<String, std::io::Error> {
    let mut f = File::open("../input")?;

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    Ok(buf)
}
