use std::collections::VecDeque;
use std::convert::{TryFrom, TryInto};

pub fn run_to_completion(prog: &[isize], input: &[isize]) -> Vec<isize> {
    let mut m = Machine {
        prog: prog.to_vec(),
        pc: 0,
        relative_base: 0,
        input: input.to_vec().into(),
    };
    let mut acc = Vec::new();

    loop {
        match m.step() {
            State::Halt => return acc,
            State::Output(o) => acc.push(o),
            _ => {}
        }
    }
}

pub struct Machine {
    prog: Vec<isize>,
    pc: usize,
    relative_base: usize,
    input: VecDeque<isize>,
}

impl Machine {
    fn ensure_dest(&mut self, dest: usize) {
        if self.prog.len() < dest + 1 {
            self.prog.resize_with(dest + 1, Default::default);
        }
    }

    fn raw_parameter(&self, n: usize) -> isize {
        self.get(self.pc + n)
    }

    fn position_param(&self, n: usize) -> usize {
        let mode = self.parameter_mode(n);
        let raw = self.raw_parameter(n);

        match mode {
            ParameterMode::Position => usize::try_from(raw).unwrap(),
            ParameterMode::Immediate => panic!("immediate mode invalid for destination param"),
            ParameterMode::Relative => (isize::try_from(self.relative_base).unwrap() + raw)
                .try_into()
                .unwrap(),
        }
    }

    /// value for parameter, decoded and dereferenced based on position mode
    fn parameter(&self, n: usize) -> isize {
        let mode = self.parameter_mode(n);
        let raw = self.raw_parameter(n);

        match mode {
            ParameterMode::Position => {
                let index = usize::try_from(raw).unwrap();
                self.get(index)
            }
            ParameterMode::Immediate => raw,
            ParameterMode::Relative => {
                let index: usize = (isize::try_from(self.relative_base).unwrap() + raw)
                    .try_into()
                    .unwrap();
                self.get(index)
            }
        }
    }

    fn get(&self, index: usize) -> isize {
        if index >= self.prog.len() {
            0
        } else {
            self.prog[index]
        }
    }

    fn set(&mut self, dest: usize, v: isize) {
        self.ensure_dest(dest);
        self.prog[dest] = v
    }

    /// decode parameter at position (params start at 1)
    fn parameter_mode(&self, n: usize) -> ParameterMode {
        let digit: isize = self.get(self.pc) / 10_isize.pow(n as u32 + 1) % 10;
        digit.into()
    }

    /// extract opcode from current pc
    fn opcode(&self) -> isize {
        let op = self.get(self.pc);
        let digit_1s = op % 10;
        let digit_10s = (op / 10) % 10;
        digit_10s * 10 + digit_1s
    }

    /// single step
    fn step(&mut self) -> State {
        let op = self.opcode();

        match op {
            1 => {
                // sum
                let left = self.parameter(1);
                let right = self.parameter(2);
                let dest_pos: usize = self.position_param(3);

                let acc = left + right;
                self.set(dest_pos, acc);
                self.pc += 4;
                State::Default
            }
            2 => {
                // product
                let left = self.parameter(1);
                let right = self.parameter(2);
                let dest_pos: usize = self.position_param(3);

                let acc = left * right;
                self.set(dest_pos, acc);
                self.pc += 4;
                State::Default
            }
            3 => {
                let acc = self.input.pop_front().expect("no input available");
                let dest_pos: usize = self.position_param(3);

                self.set(dest_pos, acc);
                self.pc += 2;
                State::Input(dest_pos)
            }
            4 => {
                let acc = self.parameter(1);

                self.pc += 2;
                State::Output(acc)
            }
            5 => {
                // jmp if true
                let acc = self.parameter(1);

                self.pc = match acc {
                    0 => self.pc + 3,
                    _ => self.parameter(2).try_into().unwrap(),
                };
                State::Default
            }
            6 => {
                // jmp if false
                let acc = self.parameter(1);

                self.pc = match acc {
                    0 => self.parameter(2).try_into().unwrap(),
                    _ => self.pc + 3,
                };
                State::Default
            }
            7 => {
                // less than
                let left = self.parameter(1);
                let right = self.parameter(2);
                let dest_pos: usize = self.position_param(3);

                let acc = if left < right { 1 } else { 0 };
                self.set(dest_pos, acc);
                self.pc += 4;
                State::Default
            }
            8 => {
                // equals
                let left = self.parameter(1);
                let right = self.parameter(2);
                let dest_pos: usize = self.position_param(3);

                let acc = if left == right { 1 } else { 0 };
                self.set(dest_pos, acc);
                self.pc += 4;
                State::Default
            }
            9 => {
                // modify relative base
                let acc = self.parameter(1);
                self.relative_base = (isize::try_from(self.relative_base).unwrap() + acc)
                    .try_into()
                    .unwrap();

                self.pc += 2;
                State::Default
            }
            99 => State::Halt,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl From<isize> for ParameterMode {
    fn from(x: isize) -> Self {
        match x {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => {
                dbg!(x);
                panic!("invalid mode")
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    Default,
    Input(usize),
    Output(isize),
    Halt,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prog_1() {
        let input: &[isize] = &[
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let output = run_to_completion(input, &[]);
        assert!(input.iter().zip(output.iter()).all(|(x, y)| x == y));
    }

    #[test]
    fn test_prog_2() {
        let input: &[isize] = &[1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let output = run_to_completion(input, &[]);
        assert!(format!("{}", output.get(0).unwrap()).len() == 16);
    }

    #[test]
    fn test_prog_3() {
        let input: &[isize] = &[104, 1125899906842624, 99];
        let output = run_to_completion(input, &[]);
        assert_eq!(1125899906842624, *output.get(0).unwrap());
    }
}
