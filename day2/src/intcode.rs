use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct IntCode {
    program: Vec<usize>
}

impl IntCode {
    pub fn run_program(&mut self) {
        let mut pc = 0;
        loop {
            let opcode = self.program[pc];
            if opcode == 99 { return; }
            let op1 = self.program[pc + 1];
            let op2 = self.program[pc + 2];
            let op3 = self.program[pc + 3];
            
            match opcode {
                1 => self.program[op3] = self.program[op1] + self.program[op2],
                2 => self.program[op3] = self.program[op1] * self.program[op2],
                _ => panic!("Invalid op-code, pc {}, opcode {}", pc, opcode)
            }
            pc += 4;
        }
    }

    pub fn new(file_name: &str) -> Self {
        let mut file = File::open(file_name).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        IntCode::parse_to_program(&buf)
    }

    fn parse_to_program(buf: &str) -> Self {
        IntCode { program: buf.split_terminator(',').map(|s| s.parse().unwrap()).collect() }
    }

    pub fn poke(&mut self, pos: usize, value: usize) {
        self.program[pos] = value;
    }

    pub fn peek(&self, pos: usize) -> usize {
        self.program[pos]
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
        let mut p = IntCode { program };

        p.run_program();

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
        let mut p = IntCode { program };

        p.run_program();

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
        let mut p = IntCode { program };

        p.run_program();

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
        let mut p = IntCode { program };

        p.run_program();

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
        let mut p = IntCode { program };

        p.run_program();

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
}