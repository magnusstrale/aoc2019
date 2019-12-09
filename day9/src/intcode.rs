use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;

pub type Cell = isize;
const ADD: usize = 1;
const MULTIPLY: usize = 2;
const INPUT: usize = 3;
const OUTPUT: usize = 4;
const JUMP_NOT_ZERO: usize = 5;
const JUMP_ZERO: usize = 6;
const STORE_LESS_THAN: usize = 7;
const STORE_EQUAL: usize = 8;
const ADJUST_RELATIVE_BASE: usize = 9;
const HALT: usize = 99;

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
    relative_base: Cell,
    debug: bool
}

impl IntCode {
    pub fn new(program: Vec<Cell>) -> Self {
        IntCode { program, pc: 0, input: VecDeque::new(), relative_base: 0, debug: true }
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
        let new_len = absolute_pos + 1;
        if new_len > self.program.len() { self.program.resize(new_len, 0); }
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
            if self.debug { println!("{}", self.disassemble()); }
            match self.opcode() {
                ADD => { 
                    let p1 = self.p(1); 
                    let p2 = self.p(2); 
                    let p3 = self.p_w(3); 
                    self.poke(p3, p1 + p2); 
                    self.pc += 4;
                },
                MULTIPLY => { 
                    let p1 = self.p(1); 
                    let p2 = self.p(2); 
                    let p3 = self.p_w(3); 
                    self.poke(p3, p1 * p2); 
                    self.pc += 4; 
                },
                INPUT => match self.input.pop_front() {
                    None => return IntCodeState::NeedInput,
                    Some(val) => {
                        let p1 = self.p_w(1);
                        self.poke(p1, val); 
                        self.pc += 2;
                    }
                },
                OUTPUT => { 
                    let result = IntCodeState::Output(self.p(1)); 
                    self.pc += 2; 
                    return result; 
                },
                JUMP_NOT_ZERO => if self.p(1) != 0 { 
                    self.pc = self.p(2) as usize; 
                } else { 
                    self.pc += 3 
                },
                JUMP_ZERO => if self.p(1) == 0 { 
                    self.pc = self.p(2) as usize; 
                } else { 
                    self.pc += 3 
                },
                STORE_LESS_THAN => { 
                    let p1 = self.p(1); 
                    let p2 = self.p(2); 
                    let p3 = self.p_w(3); 
                    self.bool_poke(p3, p1 < p2); 
                    self.pc += 4; 
                },
                STORE_EQUAL => { 
                    let p1 = self.p(1); 
                    let p2 = self.p(2); 
                    let p3 = self.p_w(3); 
                    self.bool_poke(p3, p1 == p2); 
                    self.pc += 4; 
                },
                ADJUST_RELATIVE_BASE => {
                    self.relative_base += self.p(1); 
                    self.pc += 2; 
                }
                HALT => return IntCodeState::Done,
                _ => panic!("Invalid op-code {} at pc {}", self.opcode(), self.pc)
            }
        }
    }

    fn disassemble_read_parameter(&mut self, pos: usize) -> String {
        let immediate = self.peek(self.pc + pos);
        let result = match self.mode(pos) {
            0 => format!("[{}] ({})", immediate, self.peek(immediate as usize)),
            1 => immediate.to_string(),
            2 =>  {
                let p = self.peek((immediate + self.relative_base) as usize);
                format!("[{} + {}] ({})", immediate, self.relative_base, p)
            },
            _ => panic!("Invalid parameter mode {}", self.mode(pos))
        };
        result
    }

    fn disassemble_write_parameter(&mut self, pos: usize) -> String {
        let immediate = self.peek(self.pc + pos);
        let result = match self.mode(pos) {
            0 => immediate.to_string(),
            2 => format!("{}+{} ({})", immediate, self.relative_base, (immediate + self.relative_base) as usize),
            _ => panic!("Invalid parameter mode for write {}", self.mode(pos))
        };
        result
    }

    fn disassemble(&mut self) -> String {
        let s = match self.opcode() {
            ADD => format!("ADD             {}, {} = {} -> {}",
                    self.disassemble_read_parameter(1), 
                    self.disassemble_read_parameter(2), 
                    self.p(1) + self.p(2),
                    self.disassemble_write_parameter(3)),
            MULTIPLY => format!("MULTIPLY        {}, {} = {} -> {}", 
                    self.disassemble_read_parameter(1), 
                    self.disassemble_read_parameter(2), 
                    self.p(1) * self.p(2),
                    self.disassemble_write_parameter(3)),
            INPUT => match self.input.get(0) {
                    None => format!("INPUT           NO_DATA -> {} NOP", self.disassemble_read_parameter(1)),
                    Some(&data) => format!("INPUT           {} -> {}", data, self.disassemble_read_parameter(1))
                },
            OUTPUT => format!("OUTPUT          {}", self.disassemble_read_parameter(1)),
            JUMP_NOT_ZERO => {
                format!("JUMP_NOT_ZERO   {} = {} TO {}", 
                    self.disassemble_read_parameter(1),
                    self.p(1) != 0,
                    self.disassemble_read_parameter(2))
            },
            JUMP_ZERO => {
                format!("JUMP_ZERO       {} = {} TO {}", 
                self.disassemble_read_parameter(1), 
                self.p(1) == 0,
                self.disassemble_read_parameter(2))
            },
            STORE_LESS_THAN => {
                let p1 = self.p(1); 
                let p2 = self.p(2); 
                let result = (p1 < p2) as Cell;
                format!("STORE_LESS_THAN {}, {} = {} -> {}", 
                    self.disassemble_read_parameter(1), 
                    self.disassemble_read_parameter(2),
                    result,
                    self.disassemble_write_parameter(3))
            },
            STORE_EQUAL => {
                let p1 = self.p(1); 
                let p2 = self.p(2); 
                let result = (p1 == p2) as Cell;
                format!("STORE_EQUAL     {}, {} = {} -> {}", 
                    self.disassemble_read_parameter(1), 
                    self.disassemble_read_parameter(2), 
                    result, 
                    self.disassemble_write_parameter(3))
            },
            ADJUST_RELATIVE_BASE => {
                let p1 = self.disassemble_read_parameter(1);
                let result = self.relative_base + self.p(1);
                format!("ADJUST          {}, {} = {}", self.relative_base, p1, result) 
            },
            HALT => "HALT".to_string(),
            _ => panic!("Invalid op-code {} at pc {}", self.opcode(), self.pc)
        };
        format!("{:5}: {}", self.pc, s)
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