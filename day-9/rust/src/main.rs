use std::fs::File;
use std::io::Read;

mod interpreter;

fn main() {
    let input = get_input().expect("Could not open input, does the file exist?");

    let codes = input.split_terminator(",")
        .map(|x| x.trim())
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut interpreter = interpreter::Interpreter::new(codes.clone(), vec![1]);

    while interpreter.is_running {
        interpreter.step();
    }

    println!("Part 1: {}", interpreter.last_output);

    let mut interpreter = interpreter::Interpreter::new(codes, vec![2]);

    while interpreter.is_running {
        interpreter.step();
    }

    println!("Part 2: {}", interpreter.last_output);
}


fn get_input() -> Result<String, std::io::Error> {
    let mut f = File::open("../input")?;

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    Ok(buf)
}
