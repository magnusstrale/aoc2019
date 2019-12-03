use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines};

mod wire;
use wire::Wire;

fn main() {
    let file_name = "src/day3.txt";

    let mut lines = read_file(&file_name);
    let w1 = Wire::new(&lines.next().unwrap().unwrap());
    let w2 = Wire::new(&lines.next().unwrap().unwrap());
    let mindist = w1.min_intersection_distance(&w2);
    println!("Minimum distance is {}", mindist);
}


fn read_file(file_name: &str) -> Lines<BufReader<File>> {
    let file = File::open(file_name).unwrap();
    BufReader::new(file).lines()
}
