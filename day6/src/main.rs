use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines};

mod orbit;
use orbit::*;

fn main() {
    println!("Hello, world!");
}


fn read_file(file_name: &str) -> Lines<BufReader<File>> {
    let file = File::open(file_name).unwrap();
    BufReader::new(file).lines()
}