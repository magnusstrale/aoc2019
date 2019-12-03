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

    let min_dist = w1.min_intersection_distance(&w2);
    println!("Minimum distance is {}", min_dist);

    let min_steps = w1.min_wire_steps(&w2);
    println!("Minimum steps {}", min_steps)
}


fn read_file(file_name: &str) -> Lines<BufReader<File>> {
    let file = File::open(file_name).unwrap();
    BufReader::new(file).lines()
}
