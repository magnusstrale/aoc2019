use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod orbit;
use orbit::*;

fn main() {
    let file_name = "src/day6.txt";
    let lines = read_file(file_name);
    let orbits = OrbitMap::new(&(lines.iter().map(|s| &s[..]).collect()));

    let count = orbits.total_orbits();
    println!("Total orbits {}", count);

    let distance_you_san = orbits.distance();
    println!("Distance to santa {}", distance_you_san);
}


fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    BufReader::new(file).lines().map(|l| l.unwrap()).collect()
}