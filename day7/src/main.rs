mod interpreter;
use crate::interpreter::*;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("input");

    let prog: Vec<isize> = input
        .split(",")
        .filter_map(|x| x.trim().parse::<isize>().ok())
        .collect();

    println!("part1: {}", part1(&prog)); // 12242 too low
    println!("part2: {}", part2(&prog));
}

fn part1(prog: &[isize]) -> isize {
    use itertools::Itertools;

    (0..=4)
        .permutations(5)
        // .inspect(|x| {
        //     dbg!(x);
        // })
        .map(|c| thrusters(prog, c[0], c[1], c[2], c[3], c[4]))
        // .inspect(|a| {
        //     dbg!(a);
        // })
        .max()
        .unwrap()
}

fn part2(prog: &[isize]) -> isize {
    use itertools::Itertools;

    (5..=9)
        .permutations(5)
        // .inspect(|x| {
        //     dbg!(x);
        // })
        .map(|c| feedback(prog, c[0], c[1], c[2], c[3], c[4]))
        // .inspect(|x| {
        //     dbg!(x);
        // })
        .max()
        .unwrap()
}

fn thrusters(prog: &[isize], a: isize, b: isize, c: isize, d: isize, e: isize) -> isize {
    let signal_a = 0;
    let signal_b = amplifier(prog, a, signal_a);
    let signal_c = amplifier(prog, b, signal_b);
    let signal_d = amplifier(prog, c, signal_c);
    let signal_e = amplifier(prog, d, signal_d);
    amplifier(prog, e, signal_e)
}

fn amplifier(prog: &[isize], phase: isize, signal: isize) -> isize {
    run_to_completion(prog, &vec![phase, signal]).pop().unwrap()
}

fn run_to_completion(prog: &[isize], inputs: &[isize]) -> Vec<isize> {
    let mut prog = prog.to_vec();
    let mut inputs = inputs.iter().cloned();
    let mut outputs = Vec::new();

    let mut pc = 0;

    loop {
        let (new_pc, state) = interpret(&mut prog, pc);
        pc = new_pc;

        match state {
            State::Output(out) => outputs.push(out),
            State::Input(dest) => {
                let v = inputs.next().unwrap();
                prog[dest] = v;
            }
            State::Default => {}
            State::Halt => {
                // dbg!(&outputs);
                return outputs;
            }
        }
    }
}

struct Machine {
    pc: usize,
    prog: Vec<isize>,
    input: VecDeque<isize>,
}

impl Machine {
    fn run(&mut self) -> State {
        loop {
            let (new_pc, state) = interpret(&mut self.prog, self.pc);
            self.pc = new_pc;

            match state {
                State::Output(_) => return state,
                State::Input(dest) => {
                    let v = self.input.pop_front().unwrap();
                    self.prog[dest] = v;
                }
                State::Default => {}
                State::Halt => return state,
            }
        }
    }
}

fn feedback(prog: &[isize], a: isize, b: isize, c: isize, d: isize, e: isize) -> isize {
    let mut final_o = 0;

    let mut thruster_a = Machine {
        pc: 0,
        prog: prog.to_vec(),
        input: vec![a, 0].into(),
    };
    let mut thruster_b = Machine {
        pc: 0,
        prog: prog.to_vec(),
        input: vec![b].into(),
    };
    let mut thruster_c = Machine {
        pc: 0,
        prog: prog.to_vec(),
        input: vec![c].into(),
    };
    let mut thruster_d = Machine {
        pc: 0,
        prog: prog.to_vec(),
        input: vec![d].into(),
    };
    let mut thruster_e = Machine {
        pc: 0,
        prog: prog.to_vec(),
        input: vec![e].into(),
    };

    loop {
        let s = thruster_a.run();
        let b_in = match s {
            State::Halt => return final_o,
            State::Output(o) => o,
            _ => unreachable!(),
        };
        thruster_b.input.push_back(b_in);

        let s = thruster_b.run();
        let c_in = match s {
            State::Halt => return final_o,
            State::Output(o) => o,
            _ => unreachable!(),
        };
        thruster_c.input.push_back(c_in);

        let s = thruster_c.run();
        let d_in = match s {
            State::Halt => return final_o,
            State::Output(o) => o,
            _ => unreachable!(),
        };
        thruster_d.input.push_back(d_in);

        let s = thruster_d.run();
        let e_in = match s {
            State::Halt => return final_o,
            State::Output(o) => o,
            _ => unreachable!(),
        };
        thruster_e.input.push_back(e_in);

        let s = thruster_e.run();
        let a_in = match s {
            State::Halt => return final_o,
            State::Output(o) => {
                final_o = o;
                o
            }
            _ => unreachable!(),
        };
        thruster_a.input.push_back(a_in);
    }
}
