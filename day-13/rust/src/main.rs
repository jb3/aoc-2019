#![feature(drain_filter)]
#![feature(vec_remove_item)]

use std::fs::File;
use std::io::Read;

mod interpreter;
mod game;

use game::*;

fn main() {
    let input = get_input().expect("Could not open input, does the file exist?");

    let mut codes = input.split_terminator(",")
        .map(|x| x.trim())
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut game_area = Area::new();

    let mut interpreter = interpreter::Interpreter::new(codes.clone());

    let mut outputs: Vec<i64> = Vec::new();

    while interpreter.is_running {
        interpreter.step();

        if interpreter.has_outputted {
            outputs.push(interpreter.last_output);
            interpreter.has_outputted = false;
        }
    }

    for tile in outputs.chunks_exact(3) {
        let (x, y, t) = (tile[0], tile[1], tile[2]);

        let p = Point { x, y };

        let t = Tile::from_int(t);

        game_area.set(p, t);
    }

    println!("Part 1: {}", game_area.find_count_of(Tile::Block));

    let mut game_area = Area::new();

    codes[0] = 2;

    let mut interpreter = interpreter::Interpreter::new(codes.clone());

    while interpreter.is_running {
        interpreter.step();

        interpreter.joystick = game_area.get_joystick();

        let chunks = interpreter.outputs.chunks(3).collect::<Vec<_>>();

        let mut new_chunks: Vec<i64> = Vec::new();

        for output_buf in chunks {
                if output_buf.len() == 1 {
                    new_chunks.push(output_buf[0]);
                    continue;
                }

                if output_buf.len() == 2 {
                    new_chunks.push(output_buf[0]);
                    new_chunks.push(output_buf[1]);
                    continue;
                }

                let (x, y, t) = (output_buf[0], output_buf[1], output_buf[2]);

                if x == -1 && y == 0 {
                    game_area.set_score(t);
                    continue;
                }

                if t != 3 && t != 4 {
                    continue;
                }

                new_chunks.push(output_buf[0]);
                new_chunks.push(output_buf[1]);
                new_chunks.push(output_buf[2]);

                if new_chunks.len() > 6 {
                    new_chunks.reverse();

                    for _ in 0..(3 * 50) {
                        new_chunks.pop();
                    }

                    new_chunks.reverse();
                }

                let p = Point { x, y };

                let t = Tile::from_int(t);

                game_area.set(p, t);
        }

        interpreter.outputs = new_chunks;
    }

    game_area.print_score();

}


fn get_input() -> Result<String, std::io::Error> {
    let mut f = File::open("../input")?;

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    Ok(buf)
}
