#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use super::super::intcode::*;

    fn permutations(list: Vec<Cell>, pointer: usize, acc: &mut Vec<Vec<Cell>>) {
        if pointer == list.len() {
            acc.push(list);
            return;
        }
        for i in pointer..list.len() {
            let mut permutation = list.clone();
            permutation[pointer] = list[i];
            permutation[i] = list[pointer];
            permutations(permutation, pointer + 1, acc);
        }
    }
    
    fn amplifier_output(program: &IntCode, sequence: Vec<Cell>) -> Cell {
        let mut output = 0;
        for phase in sequence {
            let mut p = program.clone();
            p.add_input(phase);
            p.add_input(output);
            output = p.run_program()[0];
        }
        output
    }
    
    pub fn max_amplifier_output(program: &IntCode) -> Cell {
        let mut max = -1;
        let mut perm = Vec::new();
        permutations(vec![0, 1, 2, 3, 4], 0, &mut perm);
        for sequence in perm.drain(..) {
            let result = amplifier_output(program, sequence);
            if result > max { max = result }
        }
        max
    }
    
    fn amplifier_output_with_feedback(program: &IntCode, sequence: Vec<Cell>) -> Cell {
        let mut amps = VecDeque::new();
        for phase in sequence {
            let mut p = program.clone();
            p.add_input(phase);
            amps.push_back(p);
        }
    
        amps[0].add_input(0);
    
        let mut last_output = 9999;
        while ! (&amps).into_iter().all(|amp| amp.is_done()) {
            let mut active_amp = amps.pop_front().unwrap();
            loop {
                match active_amp.run_slice() {
                    IntCodeState::Output(result) => {
                        amps[0].add_input(result);
                        last_output = result;
                    },
                    _ => break
                }
            }
            amps.push_back(active_amp);
        }
        last_output
    }
    
    pub fn max_feedback_amplifier_output(program: &IntCode) -> Cell {
        let mut max = -1;
        let mut perm = Vec::new();
        permutations(vec![5, 6, 7, 8, 9], 0, &mut perm);
        for sequence in perm.drain(..) {
            let result = amplifier_output_with_feedback(program, sequence);
            if result > max { max = result }
        }
        max
    }
    
    
    #[test]
    fn part1_test_relative_addressing_mode() {
        let mut p = IntCode::string_to_program("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        let actual = &p.run_program();

        assert_eq!(vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99], *actual);
    }

    #[test]
    fn part1_output_16_digit_number() {
        let mut p = IntCode::string_to_program("1102,34915192,34915192,7,4,7,99,0");
        let actual = p.run_program()[0];

        assert_eq!(1_219_070_632_396_864, actual);
    }

    #[test]
    fn part1_output_1125899906842624() {
        let mut p = IntCode::string_to_program("104,1125899906842624,99");
        let actual = p.run_program()[0];

        assert_eq!(1125899906842624, actual);
    }

    
    #[test]
    fn max_from_part2_sample1_should_give_139629729() {
        let p = IntCode::string_to_program("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
        let actual = max_feedback_amplifier_output(&p);
        assert_eq!(actual, 139629729);
    }

    #[test]
    fn max_from_part2_sample2_should_give_18216() {
        let p = IntCode::string_to_program("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");
        let actual = max_feedback_amplifier_output(&p);
        assert_eq!(actual, 18216);
    }

    #[test]
    fn max_thruster_for_sample_1_should_be_43210() {
        let p = IntCode::string_to_program("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        let actual = max_amplifier_output(&p);
        assert_eq!(actual, 43210);
    }

    #[test]
    fn max_thruster_for_sample_2_should_be_54321() {
        let p = IntCode::string_to_program("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
        let actual = max_amplifier_output(&p);
        assert_eq!(actual, 54321);
    }

    #[test]
    fn max_thruster_for_sample_3_should_be_65210() {
        let p = IntCode::string_to_program("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
        let actual = max_amplifier_output(&p);
        assert_eq!(actual, 65210);
    }
    
    #[test]
    fn given_sample_program_in_text_when_running_then_result_should_be_modified_program() {
        let program = vec![
            1,   9, 10,  3,
            2,   3, 11,  0,
            99, 30, 40, 50];
        let mut p = IntCode::new(program);
        p.run_slice();
        let final_state = vec![
            3500, 9, 10, 70,
            2, 3, 11, 0,
            99, 30, 40, 50];
        assert_eq!(p.program, final_state);
    }

    #[test]
    fn given_sample_1_when_running_then_result_should_be_modified_program() {
        let program = vec![
            1, 0, 0,  0,
            99];
        let mut p = IntCode::new(program);
        p.run_slice();
        let final_state = vec![
            2, 0, 0, 0,
            99];
        assert_eq!(p.program, final_state);
    }

    #[test]
    fn given_sample_2_when_running_then_result_should_be_modified_program() {
        let program = vec![
            2, 3, 0, 3,
            99];
        let mut p = IntCode::new(program);
        p.run_slice();
        let final_state = vec![
            2, 3, 0, 6,
            99];
        assert_eq!(p.program, final_state);
    }

    #[test]
    fn given_sample_3_when_running_then_result_should_be_modified_program() {
        let program = vec![
            2, 4, 4, 5,
            99, 0];
        let mut p = IntCode::new(program);
        p.run_slice();
        let final_state = vec![
            2, 4, 4, 5,
            99, 9801];
        assert_eq!(p.program, final_state);
    }

    #[test]
    fn given_sample_4_when_running_then_result_should_be_modified_program() {
        let program = vec![
            1, 1, 1, 4,
            99, 5, 6, 0,
            99];
        let mut p = IntCode::new(program);
        p.run_slice();
        let final_state = vec![
            30, 1, 1, 4,
            2, 5, 6, 0,
            99];
        assert_eq!(p.program, final_state);
    }

    #[test]
    fn given_string_when_parse_to_program_then_should_split_on_comma_into_program() {
        let p = IntCode::string_to_program("1,2,3,4");
        assert_eq!(p.program, vec![1, 2, 3, 4]);
    }

    #[test]
    fn given_opcode_3_program_with_input_52_should_write_52_to_end() {
        let mut p = IntCode::string_to_program("3,3,99,0");
        p.add_input(52);
        p.run_slice();
        assert_eq!(52, p.program[3]);
    }

    #[test]
    fn given_mixed_parameter_mode_should_write_99_at_end() {
        let mut p = IntCode::string_to_program("1002,4,3,4,33");
        p.run_slice();
        assert_eq!(99, p.program[4]);
    }

    #[test]
    fn position_mode_jump_program_should_give_0_for_input_0() {
        let mut p = IntCode::string_to_program("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        p.add_input(0);
        let output = p.run_program()[0];
        assert_eq!(0, output);
    }

    #[test]
    fn position_mode_jump_program_should_give_1_for_input_5() {
        let mut p = IntCode::string_to_program("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        p.add_input(5);
        let output = p.run_program()[0];
        assert_eq!(1, output);
    }

    #[test]
    fn immediate_mode_jump_program_should_give_0_for_input_0() {
        let mut p = IntCode::string_to_program("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
        p.add_input(0);
        let output = p.run_program()[0];
        assert_eq!(0, output);
    }

    #[test]
    fn immediate_mode_jump_program_should_give_1_for_input_5() {
        let mut p = IntCode::string_to_program("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
        p.add_input(5);
        let output = p.run_program()[0];
        assert_eq!(1, output);
    }

    #[test]
    fn check_8_program_should_give_999_for_input_below_8() {
        let mut p = IntCode::string_to_program("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        p.add_input(7);
        let output = p.run_program()[0];
        assert_eq!(999, output);
    }

    #[test]
    fn check_8_program_should_give_1000_for_input_8() {
        let mut p = IntCode::string_to_program("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        p.add_input(8);
        let output = p.run_program()[0];
        assert_eq!(1000, output);
    }
    
    #[test]
    fn check_8_program_should_give_1001_for_input_above_8() {
        let mut p = IntCode::string_to_program("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        p.add_input(9);
        let output = p.run_program()[0];
        assert_eq!(1001, output);
    }
}
