use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod asteroid_map;
use asteroid_map::*;

fn main() {
    let input = read_file("src/day10.txt");
    let mut map = Map::new(&(input.iter().map(|s| &s[..]).collect()));

    let (pos, count) = map.find_best_location();
    println!("Found {} asteroids from {:?}", count, pos);

    let vaporized = map.vaporize_asteroids(pos);
    let a200 = vaporized[199];
    println!("200 at {:?} gives {}", a200, a200.x * 100 + a200.y);
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    BufReader::new(file).lines().map(|l| l.unwrap()).collect()
}