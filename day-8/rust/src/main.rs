use std::io::Read;
use std::fs::File;
use std::process::{Command, Stdio};

use image::ImageBuffer;

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

    let mut imgbuf = ImageBuffer::new((IMAGE_WIDTH as u32) * 10, (IMAGE_HEIGHT as u32) * 10);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if constructed_image[y as usize / 10][x as usize / 10] == 1 {
            *pixel = image::Luma([0]);
        } else {
            *pixel = image::Luma([255]);
        }
    }

    let ocrad = Command::new("ocrad")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Could not start ocrad, is it installed?");

    let buf: &[u16] = &imgbuf.into_vec()[..];

    image::pnm::PNMEncoder::new(ocrad.stdin.unwrap())
        .with_subtype(image::pnm::PNMSubtype::Bitmap(image::pnm::SampleEncoding::Binary))
        .encode(buf, (IMAGE_WIDTH * 10) as u32, (IMAGE_HEIGHT * 10) as u32, image::Gray(8))
        .unwrap();

    let mut buf = String::new();

    ocrad.stdout.unwrap().read_to_string(&mut buf).unwrap();

    println!("Part 2: {}", buf.trim());
}

fn get_input() -> Vec<i32> {
    let mut f = File::open("../input").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    buf = buf.trim_end().to_string();

    buf.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>()
}
