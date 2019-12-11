use std::fs::File;
use std::io::Read;

mod interpreter;
mod canvas;
mod letters;

use canvas::{Canvas, Colour, Location};

fn main() {
    let input = get_input().expect("Could not open input, does the file exist?");

    let codes = input.split_terminator(",")
        .map(|x| x.trim())
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut interpreter = interpreter::Interpreter::new(codes.clone(), vec![]);

    let mut cnvs = canvas::Canvas::new();

    paint(&mut cnvs, &mut interpreter);

    println!("Part 1: {}", cnvs.history.len());

    let mut interpreter = interpreter::Interpreter::new(codes.clone(), vec![]);
    let mut cnvs = canvas::Canvas::new();

    cnvs.set_colour(&Location {x: 0, y: 0}, canvas::Colour::White);

    paint(&mut cnvs, &mut interpreter);

    let mut data: Vec<(&Location, &Colour)> = cnvs.painted.iter().collect();

    data.sort_by_key(|a| a.0.x);

    let xr = data.first().unwrap().0.x..data.last().unwrap().0.x;

    data.sort_by_key(|a| a.0.y);


    let min_y = data.first().unwrap().0.y;

    let mut cols: Vec<Vec<bool>> = Vec::new();

    for x in xr {
        let yr = data.first().unwrap().0.y..=data.last().unwrap().0.y;
        let mut row: Vec<bool> = Vec::new();

        for y in yr {
            if cnvs.get_colour(&Location { x: x, y: min_y - y }) == Colour::White {
                row.push(true);
            } else {
                row.push(false);
            }
        }

        cols.push(row);
    }

    let mut res = String::new();

    for group in cols.chunks_exact(5) {
        let mut group = group.to_vec();

        group[0] = group[1].clone();
        group[1] = group[2].clone();
        group[2] = group[3].clone();
        group[3] = group[4].clone();
        group[4] = vec![false, false, false, false, false, false];

        let l = letters::find_letter(group);

        res.push(l);
    }

    println!("Part 2: {}", res);
}

fn paint(canvas: &mut Canvas, interpreter: &mut interpreter::Interpreter) {
    let mut current_location: Location = (0, 0).into();
    let mut heading = 0;

    'outer: while interpreter.is_running {

        if canvas.get_colour(&current_location) == Colour::Black {
            interpreter.add_input(0);
        } else {
            interpreter.add_input(1);
        }

        while !interpreter.has_outputted {
            interpreter.step();

            if interpreter.is_running == false {
                break 'outer;
            }
        }

        interpreter.has_outputted = false;

        let colour = if interpreter.last_output == 0 {
            Colour::Black
        } else {
            Colour::White
        };

        canvas.set_colour(&current_location, colour);

        while !interpreter.has_outputted {
            interpreter.step();
        }

        interpreter.has_outputted = false;

        if interpreter.last_output == 0 {
            heading = turn_left(heading);
        } else {
            heading = turn_right(heading);
        }

        current_location = match heading {
            0 => (current_location.x, current_location.y + 1),
            90 => (current_location.x + 1, current_location.y),
            180 => (current_location.x, current_location.y - 1),
            270 => (current_location.x - 1, current_location.y),
            _ => panic!("{}", heading % 360)
        }.into();
    }
}

fn turn_right(heading: i64) -> i64 {
    match heading {
        0 => 90,
        90 => 180,
        180 => 270,
        270 => 0,
        _ => panic!()
    }
}

fn turn_left(heading: i64) -> i64 {
    match heading {
        0 => 270,
        270 => 180,
        180 => 90,
        90 => 0,
        _ => panic!()
    }
}

fn get_input() -> Result<String, std::io::Error> {
    let mut f = File::open("../input")?;

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    Ok(buf)
}
