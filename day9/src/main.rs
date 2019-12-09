pub mod intcode;
pub mod old_intcode_tests;

use intcode::*;

fn main() {
    let input_file = "src/day9.txt";
    let original = IntCode::file_to_program(&input_file);

    let mut program = original.clone();
    program.add_input(1);
    let result = program.run_program();

    println!("BOOST keycode for test run is {:?}", result);

    let mut program = original.clone();
    program.add_input(2);
    let result = program.run_program();

    println!("BOOST keycode for sensor boost mode is {:?}", result);
}

