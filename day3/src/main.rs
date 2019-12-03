use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines};

mod wire;
use wire::Wire;

fn main() {
    let file_name = "src/day3.txt";

    let lines = read_file(&file_name);
    let mut wires = Vec::new();
    for line in lines {
        wires.push(Wire::new(&line.unwrap()));
    }
    let w1 = &wires[0];
    let w2 = &wires[1];
    let mindist = w1.min_intersection_distance(w2);
    println!("Minimum distance is {}", mindist);
}


fn read_file(file_name: &str) -> Lines<BufReader<File>> {
    let file = File::open(file_name).unwrap();
    BufReader::new(file).lines()
}
