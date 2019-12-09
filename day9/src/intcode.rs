use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;

pub type Cell = isize;

#[derive(Debug, PartialEq, Clone)]
pub enum IntCodeState {
    Done,
    Output(Cell),
    NeedInput
}

#[derive(Debug, Clone)]
pub struct IntCode {
    pub program: Vec<Cell>,
    pc: usize,
    input: VecDeque<Cell>,
    relative_base: Cell
}

impl IntCode {
    pub fn new(program: Vec<Cell>) -> Self {
        IntCode { program, pc: 0, input: VecDeque::new(), relative_base: 0 }
    }

    pub fn file_to_program(file_name: &str) -> Self {
        let mut file = File::open(file_name).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        IntCode::string_to_program(&buf)
    }

    pub fn string_to_program(buf: &str) -> Self {
        IntCode::new(buf.split_terminator(',').map(|s| s.parse().unwrap()).collect())
    }

    pub fn add_input(&mut self, input: Cell) {
        self.input.push_back(input);
    }

    pub fn is_done(&self) -> bool {
        self.opcode() == 99
    }

    fn opcode(&self) -> usize {
        // Bypass peek to allow non-mutable use of self
        (self.program[self.pc] % 100) as usize
    }

    fn allocate(&mut self, absolute_pos: usize) {
        while self.program.len() <= absolute_pos {
            self.program.push(0);
        }
    }

    fn peek(&mut self, absolute_pos: usize) -> Cell {
        self.allocate(absolute_pos);
        self.program[absolute_pos]
    }

    fn mode(&self, pos: usize) -> usize {
        let full_opcode = self.program[self.pc];
        (full_opcode / 10_isize.pow(1 + pos as u32) % 10) as usize
    }

    fn p(&mut self, pos: usize) -> Cell {
        let immediate = self.peek(self.pc + pos);
        let result = match self.mode(pos) {
            0 => self.peek(immediate as usize),
            1 => immediate,
            2 => self.peek((immediate + self.relative_base) as usize),
            _ => panic!("Invalid parameter mode {}", self.mode(pos))
        };
        result
    }

    fn p_w(&mut self, pos: usize) -> usize {
        let immediate = self.peek(self.pc + pos);
        let result = if self.mode(pos) == 2 { immediate + self.relative_base } else { immediate };

        result as usize
    }

    fn poke(&mut self, pos: usize, value: Cell) {
        self.allocate(pos);
        self.program[pos] = value;
    }

    fn bool_poke(&mut self, pos: usize, value: bool) {
        self.poke(pos, value as Cell);
    }

    pub fn run_program(&mut self) -> Vec<Cell> {
        let mut output = Vec::new();
        loop {
            match self.run_slice() {
                IntCodeState::Done => return output,
                IntCodeState::NeedInput => panic!("Not enough input data"),
                IntCodeState::Output(result) => output.push(result)
            }
        }
    }

    pub fn run_slice(&mut self) -> IntCodeState {
        loop {
            match self.opcode() {
                1 => { 
                    let p1 = self.p(1); 
                    let p2 = self.p(2); 
                    let p3 = self.p_w(3); 
                    self.poke(p3, p1 + p2); 
                    self.pc += 4;
                },
                2 => { 
                    let p1 = self.p(1); 
                    let p2 = self.p(2); 
                    let p3 = self.p_w(3); 
                    self.poke(p3, p1 * p2); 
                    self.pc += 4; 
                },
                3 => match self.input.pop_front() {
                    None => return IntCodeState::NeedInput,
                    Some(val) => {
                        let p1 = self.p_w(1);
                        self.poke(p1, val); 
                        self.pc += 2;
                    }
                },
                4 => { 
                    let result = IntCodeState::Output(self.p(1)); 
                    self.pc += 2; 
                    return result; 
                },
                5 => if self.p(1) != 0 { 
                    self.pc = self.p(2) as usize; 
                } else { 
                    self.pc += 3 
                },
                6 => if self.p(1) == 0 { 
                    self.pc = self.p(2) as usize; 
                } else { 
                    self.pc += 3 
                },
                7 => { 
                    let p1 = self.p(1); 
                    let p2 = self.p(2); 
                    let p3 = self.p_w(3); 
                    self.bool_poke(p3, p1 < p2); 
                    self.pc += 4; 
                },
                8 => { 
                    let p1 = self.p(1); 
                    let p2 = self.p(2); 
                    let p3 = self.p_w(3); 
                    self.bool_poke(p3, p1 == p2); 
                    self.pc += 4; 
                },
                9 => {
                    self.relative_base += self.p(1); 
                    self.pc += 2; 
                }
                99 => return IntCodeState::Done,
                _ => panic!("Invalid op-code {} at pc {}", self.opcode(), self.pc)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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
}