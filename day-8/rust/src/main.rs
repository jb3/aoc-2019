use std::io::Read;
use std::fs::File;

mod letters;

const IMAGE_WIDTH: usize = 25;
const IMAGE_HEIGHT: usize = 6;

fn main() {
    let input = get_input();

    let pixels_per_layer = IMAGE_WIDTH * IMAGE_HEIGHT;

    let layer_data: Vec<Vec<i32>> = input.chunks(pixels_per_layer).map(|x| Vec::from(x)).collect();

    let mut sorted = layer_data.clone();

    sorted.sort_by_key(|s| s.iter().filter(|&n| *n == 0).count());

    let fewest_zeros = sorted.first().unwrap();

    let num_1s = fewest_zeros.iter().filter(|&n| *n == 1).count();
    let num_2s = fewest_zeros.iter().filter(|&n| *n == 2).count();

    println!("Part 1: {}", num_1s * num_2s);

    let mut layers: Vec<Vec<Vec<i32>>> = Vec::new();

    for layer in layer_data {
        let rows: Vec<Vec<i32>> = layer.chunks(IMAGE_WIDTH).map(|x| Vec::from(x)).collect();
        layers.push(rows);
    }

    let mut constructed_image: Vec<Vec<i32>> = Vec::new();

    for x in 0..IMAGE_HEIGHT {
        constructed_image.push(Vec::new());
        for y in 0..IMAGE_WIDTH {
            for layer in &layers {
                if layer[x][y] != 2 {
                    constructed_image[x].push(layer[x][y]);
                    break;
                }
            }
        }
    }

    let mut columns: Vec<Vec<bool>> = Vec::new();

    for y in 0..IMAGE_WIDTH {
        let mut col: Vec<bool> = Vec::new();
        for x in 0..IMAGE_HEIGHT {
            let pix = constructed_image[x][y];
            col.push(pix == 1);
        }
        columns.push(col);
    }

    let mut solution = String::new();

    for letter in columns.chunks(5) {
        solution.push(letters::find_letter(letter.to_vec()));
    }

    println!("Part 2: {}", solution);
}

fn get_input() -> Vec<i32> {
    let mut f = File::open("../input").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    buf = buf.trim_end().to_string();

    buf.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>()
}
