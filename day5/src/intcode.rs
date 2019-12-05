use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
pub struct IntCode {
    program: Vec<i64>,
    pc: usize
}

impl IntCode {
    const INSTRUCTION_LENGTHS: [usize; 8] = [4, 4, 2, 2, 3, 3, 4, 4];

    pub fn new(file_name: &str) -> Self {
        let mut file = File::open(file_name).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        IntCode::parse_to_program(&buf)
    }

    fn opcode(&self) -> usize {
        (self.program[self.pc] % 100) as usize
    }

    fn instruction_length(&self) -> usize {
        IntCode::INSTRUCTION_LENGTHS[self.opcode() - 1]
    }

    fn p(&self, pos: usize) -> i64 {
        let full_opcode = self.program[self.pc];
        let mode = match pos {
            1 => full_opcode / 100 % 10,
            2 => full_opcode / 1_000 % 10,
            3 => full_opcode / 10_000 % 10,
            _ => panic!("Invalid parameter index {}", pos)
        };
        if mode != 0 && mode != 1 { panic!("Invalid mode for pc {} full_opcode {}", self.pc, full_opcode); }
        let immediate = self.program[self.pc + pos];
        if mode == 1 { immediate } else { self.program[immediate as usize] }
    }

    fn p_w(&self, pos: usize) -> usize {
        self.program[self.pc + pos] as usize
    }

    pub fn run_program(&mut self, input: i64) -> i64 {
        let mut latest_output = 0;
        self.pc = 0;
        loop {
            let opcode = self.opcode();
            if opcode == 99 { return latest_output; }
            //println!("pc: {} opcode: {}  [{}, {}, {}]", self.pc, self.program[self.pc], self.program[self.pc + 1], self.program[self.pc + 2], self.program[self.pc + 3]);
            let mut next_pc = self.pc + self.instruction_length();
            match opcode {
                1 => self.poke(self.p_w(3), self.p(1) + self.p(2)),
                2 => self.poke(self.p_w(3), self.p(1) * self.p(2)),
                3 => self.poke(self.p_w(1), input),
                4 => { latest_output = self.p(1); println!("Output: {}", latest_output); },
                5 => if self.p(1) != 0 { next_pc = self.p(2) as usize; },
                6 => if self.p(1) == 0 { next_pc = self.p(2) as usize; },
                7 => self.write_one_if_true(self.p(1) < self.p(2)),
                8 => self.write_one_if_true(self.p(1) == self.p(2)),
                _ => panic!("Invalid op-code {} at pc {}", opcode, self.pc)
            }
            self.pc = next_pc;
        }
    }

    fn write_one_if_true(&mut self, cond: bool) {
        let val = if cond {1} else {0};
        self.poke(self.p_w(3), val);
    }

    fn parse_to_program(buf: &str) -> Self {
        IntCode { program: buf.split_terminator(',').map(|s| s.parse().unwrap()).collect(), pc: 0 }
    }

    fn poke(&mut self, pos: usize, value: i64) {
        //println!("Poke at {}, changing from {} to {}", pos, self.program[pos], value);
        self.program[pos] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_sample_program_in_text_when_running_then_result_should_be_modified_program() {
        let program = vec![
            1,   9, 10,  3,
            2,   3, 11,  0,
            99, 30, 40, 50];
        let mut p = IntCode { program, pc: 0 };
        p.run_program(0);
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
        let mut p = IntCode { program, pc: 0 };
        p.run_program(0);
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
        let mut p = IntCode { program, pc: 0 };
        p.run_program(0);
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
        let mut p = IntCode { program, pc: 0 };
        p.run_program(0);
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
        let mut p = IntCode { program, pc: 0 };
        p.run_program(0);
        let final_state = vec![
            30, 1, 1, 4,
            2, 5, 6, 0,
            99];
        assert_eq!(p.program, final_state);
    }

    #[test]
    fn given_string_when_parse_to_program_then_should_split_on_comma_into_program() {
        let p = IntCode::parse_to_program("1,2,3,4");
        assert_eq!(p.program, vec![1, 2, 3, 4]);
    }

    #[test]
    fn when_poking_999_into_pos0_then_pos0_should_hold_999() {
        let mut p = IntCode::parse_to_program("0,0,0,0,0,0");
        p.poke(0, 999);
        assert_eq!(999, p.program[0]);
    }

    #[test]
    fn given_opcode_3_program_with_input_52_should_write_52_to_end() {
        let mut p = IntCode::parse_to_program("3,3,99,0");
        p.run_program(52);
        assert_eq!(52, p.program[3]);
    }

    #[test]
    fn given_mixed_parameter_mode_should_write_99_at_end() {
        let mut p = IntCode::parse_to_program("1002,4,3,4,33");
        p.run_program(0);
        assert_eq!(99, p.program[4]);
    }

    #[test]
    fn position_mode_jump_program_should_give_0_or_input_0() {
        let mut p = IntCode::parse_to_program("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        let output = p.run_program(0);
        assert_eq!(0, output);
    }

    #[test]
    fn position_mode_jump_program_should_give_1_for_input_5() {
        let mut p = IntCode::parse_to_program("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        let output = p.run_program(5);
        assert_eq!(1, output);
    }

    #[test]
    fn immediate_mode_jump_program_should_give_0_for_input_0() {
        let mut p = IntCode::parse_to_program("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
        let output = p.run_program(0);
        assert_eq!(0, output);
    }

    #[test]
    fn immediate_mode_jump_program_should_give_1_for_input_5() {
        let mut p = IntCode::parse_to_program("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
        let output = p.run_program(5);
        assert_eq!(1, output);
    }

    #[test]
    fn check_8_program_should_give_999_for_input_below_8() {
        let mut p = IntCode::parse_to_program("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        let output = p.run_program(7);
        assert_eq!(999, output);
    }

    #[test]
    fn check_8_program_should_give_1000_for_input_8() {
        let mut p = IntCode::parse_to_program("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        let output = p.run_program(8);
        assert_eq!(1000, output);
    }
    #[test]
    fn check_8_program_should_give_1001_for_input_above_8() {
        let mut p = IntCode::parse_to_program("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        let output = p.run_program(9);
        assert_eq!(1001, output);
    }
}