use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
pub struct IntCode {
    program: Vec<i64>,
    pc: usize
}

impl IntCode {
    pub fn new(file_name: &str) -> Self {
        let mut file = File::open(file_name).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        IntCode::parse_to_program(&buf)
    }

    fn opcode(&self) -> i64 {
        self.program[self.pc] % 100
    }

    fn instruction_length(&self) -> usize {
        let opcode = self.opcode();
        if opcode == 1 || opcode == 2 { 4 } else { 2 }
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
        if mode == 1 { immediate } else { self.peek(immediate) }
    }

    fn p_w(&self, pos: usize) -> usize {
        self.program[self.pc + pos] as usize
    }

    pub fn run_program(&mut self, system_id: i64) {
        self.pc = 0;
        loop {
            let opcode = self.opcode();
            if opcode == 99 { return; }
            //println!("pc: {} opcode: {}  [{}, {}, {}]", self.pc, self.program[self.pc], self.program[self.pc + 1], self.program[self.pc + 2], self.program[self.pc + 3]);
            
            match opcode {
                1 => self.poke(self.p_w(3), self.p(1) + self.p(2)),
                2 => self.poke(self.p_w(3), self.p(1) * self.p(2)),
                3 => self.poke(self.p_w(1), system_id),
                4 => println!("Output: {}", self.p(1)),
                _ => panic!("Invalid op-code {} at pc {}", opcode, self.pc)
            }
            self.pc += self.instruction_length();
        }
    }

    fn parse_to_program(buf: &str) -> Self {
        IntCode { program: buf.split_terminator(',').map(|s| s.parse().unwrap()).collect(), pc: 0 }
    }

    fn poke(&mut self, pos: usize, value: i64) {
        // println!("Poke at {}, changing from {} to {}", pos, self.program[pos], value);
        self.program[pos] = value;
    }

    fn peek(&self, pos: i64) -> i64 {
        self.program[pos as usize]
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
    fn given_all_zero_program_when_poking_999_into_pos0_then_peeking_pos0_should_give_999() {
        let mut p = IntCode::parse_to_program("0,0,0,0,0,0");
        p.poke(0,999);
        assert_eq!(999, p.peek(0));
    }

    #[test]
    fn given_opcode_3_program_with_sysid_52_should_write_52_to_end() {
        let mut p = IntCode::parse_to_program("3,3,99,0");
        p.run_program(52);
        assert_eq!(52, p.peek(3));
    }

    #[test]
    fn given_mixed_parameter_mode_should_write_99_at_end() {
        let mut p = IntCode::parse_to_program("1002,4,3,4,33");
        p.run_program(0);
        assert_eq!(99, p.peek(4));
    }
}