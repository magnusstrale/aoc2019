pub mod intcode;
use intcode::*;

fn main() {
    let input_file = "src/day2.txt";
    let mut p = IntCode::new(&input_file);
    p.poke(1, 12);
    p.poke(2, 2);
    p.run_program();
    println!("Result at position 0 is {}", p.peek(0));
}
