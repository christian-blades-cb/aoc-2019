mod interpreter;
use crate::interpreter::{interpret, State};

fn main() {
    let input = include_str!("input");

    let prog: Vec<isize> = input
        .split(",")
        .filter_map(|x| x.trim().parse::<isize>().ok())
        .collect();

    // println!("part1: {}", part1(&prog)); // 223 too low

    println!("part2: {}", part2(&prog));
}

fn part1(input: &[isize]) -> isize {
    let mut prog = input.to_vec();
    let mut inputs = vec![1];
    let mut outputs = Vec::new();

    let mut pc = 0;

    loop {
        let (new_pc, state) = interpret(&mut prog, pc);
        pc = new_pc;

        match state {
            State::Output(out) => outputs.push(out),
            State::Input(dest) => {
                let v = inputs.pop().unwrap();
                prog[dest] = v;
            }
            State::Default => {}
            State::Halt => {
                dbg!(&outputs);
                return outputs.pop().unwrap();
            }
        }
    }
}

fn part2(input: &[isize]) -> isize {
    let mut prog = input.to_vec();
    let mut inputs = vec![5];
    let mut outputs = Vec::new();

    let mut pc = 0;

    loop {
        dbg!(&pc, prog[pc]);
        let (new_pc, state) = interpret(&mut prog, pc);
        pc = new_pc;

        match state {
            State::Output(out) => outputs.push(out),
            State::Input(dest) => {
                let v = inputs.pop().unwrap();
                prog[dest] = v;
            }
            State::Default => {}
            State::Halt => {
                dbg!(&outputs);
                return outputs.pop().unwrap();
            }
        }
    }
}
