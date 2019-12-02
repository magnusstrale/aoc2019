use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines};

pub mod fuel_counter;
use fuel_counter::*;

fn main() {
    let input_file = "src/day1.txt";
    let total_fuel = calculate_fuel_part1(&input_file);
    println!("Fuel needed - part1: {}", total_fuel);
    let adjusted_fuel = calculate_fuel_part2(&input_file);
    println!("Fuel needed - part2: {}", adjusted_fuel);
}

fn calculate_fuel_part1(file_name: &str) -> i64 {
    let lines = read_file(file_name);
    lines.map(|line| calculate_fuel(line.unwrap().parse().unwrap())).sum()
}

fn calculate_fuel_part2(file_name: &str) -> i64 {
    let lines = read_file(file_name);
    lines.map(|line| calculate_adjusted_fuel(line.unwrap().parse().unwrap())).sum()
}

fn read_file(file_name: &str) -> Lines<BufReader<File>> {
    let file = File::open(file_name).unwrap();
    BufReader::new(file).lines()
}
