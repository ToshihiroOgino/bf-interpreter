use std::io::Write;

use crate::{expression::Expression, machine::Machine};

type CodePointer = usize;

pub struct Interpreter {
    code: Vec<Expression>,
    code_pointer: CodePointer,
    machine: Machine,
    pointer_stack: Vec<CodePointer>,
}

fn convert_to_ascii(input: u8) -> char {
    if input >= 128 {
        panic!("Input is not a valid ASCII character");
    }
    input as char
}

fn get_char() -> u8 {
    print!("Input: ");
    std::io::stdout().flush().expect("Failed to flush stdout");
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");
    let ch = buf.chars().next().unwrap_or('\0');
    if ch.is_ascii() {
        ch as u8
    } else {
        panic!("Input is not a valid ASCII character");
    }
}

fn print_char(ch: char) {
    println!("Output: {}({})", ch, ch as u8);
}

impl Interpreter {
    pub fn new(sourcecode: &str) -> Self {
        Interpreter {
            code: Expression::from_string(sourcecode),
            code_pointer: 0,
            machine: Machine::new(),
            pointer_stack: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        while self.code_pointer < self.code.len() {
            self.step();
        }
    }

    fn step(&mut self) {
        let expr = self.code.get(self.code_pointer).unwrap();
        match expr {
            Expression::Increment => {
                self.machine.increment().unwrap();
                self.code_pointer += 1;
            }
            Expression::Decrement => {
                self.machine.decrement().unwrap();
                self.code_pointer += 1;
            }
            Expression::MoveRight => {
                self.machine.move_right().unwrap();
                self.code_pointer += 1;
            }
            Expression::MoveLeft => {
                self.machine.move_left().unwrap();
                self.code_pointer += 1;
            }
            Expression::LoopBegin => {
                if self.machine.get().unwrap() == 0 {
                    if let Some(end) = self.find_matching_bracket(self.code_pointer) {
                        self.code_pointer = end + 1;
                    } else {
                        panic!("Could not find matching loop end");
                    }
                } else {
                    self.pointer_stack.push(self.code_pointer);
                    self.code_pointer += 1;
                }
            }
            Expression::LoopEnd => {
                let loop_begin = self.pointer_stack.pop().unwrap();
                let head_val = self.machine.get().unwrap();
                if head_val != 0 {
                    self.code_pointer = loop_begin;
                } else {
                    self.code_pointer += 1;
                }
            }
            Expression::Output => {
                let output = convert_to_ascii(self.machine.get().unwrap());
                print_char(output);
                self.code_pointer += 1;
            }
            Expression::Input => {
                let input = get_char();
                self.machine.set(input).unwrap();
                self.code_pointer += 1;
            }
        }
    }

    fn find_matching_bracket(&self, start: usize) -> Option<usize> {
        let mut depth = 1;
        for i in start + 1..self.code.len() {
            match self.code[i] {
                Expression::LoopBegin => depth += 1,
                Expression::LoopEnd => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(i);
                    }
                }
                _ => {}
            }
        }
        None
    }
}
