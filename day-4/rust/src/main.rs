use std::ops::Range;
use std::fs::File;
use std::io::Read;

fn main() {
    let range = get_range();

    let mut meets_part_one = 0;

    for i in range {
        if number_meets_part_one(i) {
            meets_part_one += 1;
        }
    }

    println!("Part 1: {}", meets_part_one);

    let mut meets_part_two = 0;

    let range = get_range();

    for i in range {
        if number_meets_part_two(i) {
            meets_part_two += 1;
        }
    }

    println!("Part 2: {}", meets_part_two);
}

fn get_range() -> Range<i32> {
    let mut f = File::open("../input").expect("Could not open input file");

    let mut buffer = String::new();
    f.read_to_string(&mut buffer).expect("Could not read input file");

    let split = buffer.split("-");

    let parsed = split.map(|splt| splt.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let (lower, upper) = (parsed[0], parsed[1]);

    lower..upper
}

fn number_meets_part_one(number: i32) -> bool {
    if not_decreasing(number) {
        if has_double_integer(number) {
            return true;
        }
    }

    false
}

fn number_meets_part_two(number: i32) -> bool {
    if not_decreasing(number){
        if has_no_larger_group(number) {
            return true;
        }
    }

    false
}

fn has_double_integer(number: i32) -> bool {
    let digits = get_digits(number);

    let rle = rle(digits);

    let groups = rle.iter().map(|x| x.0 > 1).collect::<Vec<bool>>();

    if groups.contains(&true) {
        return true;
    }

    false
}

fn rle(list: Vec<i32>) -> Vec<(i32, i32)> {
    let mut rle: Vec<(i32, i32)> = Vec::new();

    for digit in list.iter() {
        if let Some(x) = rle.last() {
            if x.1 == *digit {
                let len = rle.len();
                rle[len - 1] = (x.0 + 1, *digit);
            } else {
                rle.push((1, *digit));
            }
        } else {
            rle.push((1, *digit));
        }
    }

    rle
}

fn has_no_larger_group(number: i32) -> bool {
    let digits = get_digits(number);

    let rle = rle(digits);

    let groups = rle.iter().map(|x| x.0).collect::<Vec<i32>>();

    if groups.contains(&2) {
        return true;
    }

    false
}

pub fn get_digits(number: i32) -> Vec<i32> {
    number.to_string()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| (x as i32))
        .collect()
}

fn not_decreasing(number: i32) -> bool {
    let string = number.to_string();

    let mut last_number = 0;

    for character in string.chars() {
        let digit = character.to_digit(10).unwrap();


        if digit < last_number {
            return false;
        }

        last_number = digit;
    }

    true
}
