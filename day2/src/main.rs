pub mod intcode;
use intcode::*;

const EXPECTED: usize = 19690720;

fn main() {
    let input_file = "src/day2.txt";
    let original = IntCode::new(&input_file);

    let mut p = original.clone();
    let result = p.run_patched_program(12, 2);
    println!("Result is {}", result);

    let (noun, verb) = find_verb_noun(&original);

    println!("Noun {}, verb {} gives code {}", noun, verb, 100 * noun + verb);
}

fn find_verb_noun(program: &IntCode) -> (usize, usize) {
    for v in 0..=99 {
        for n in 0..=99 {
            let mut p = program.clone();
            if p.run_patched_program(n, v) == EXPECTED { return (n, v); }
        }
    }

    (0, 0)
}
