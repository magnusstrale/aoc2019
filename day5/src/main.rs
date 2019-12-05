pub mod intcode;
use intcode::*;

fn main() {
    let input_file = "src/day5.txt";
    let original = IntCode::new(&input_file);

    println!("Part 1 - test AC");
    original.clone().run_program(1);

    println!("\nPart 2 - test thermal radiator");
    original.clone().run_program(5);
}

