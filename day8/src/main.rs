use std::fs::File;
use std::io::prelude::*;

mod image;
use image::*;

fn main() {
    let file_name = "src/day8.txt";
    let mut file = File::open(file_name).expect("Error opening file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("File error");

    let mut image = Image::new(25, 6);
    image.load(&buffer);

    let l = image.min_count_layer('0');
    let result = image.char_count_layer('1', l) * image.char_count_layer('2', l);
    println!("The product is {}", result);

    println!("The message is ");
    let ls = image.decode_to_lines();
    for l in ls {
        println!("{}", l);
    }
}
