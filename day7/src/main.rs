pub mod intcode;
use intcode::*;

fn main() {
    let input_file = "src/day7.txt";
    let original = IntCode::new(&input_file);

    let output = max_amplifier_output(&original);
    println!("Max thruster {}", output);

    let output = max_feedback_amplifier_output(&original);
    println!("Max thruster w feedback loop {}", output);
}

