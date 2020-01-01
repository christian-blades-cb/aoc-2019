mod interpreter;
use crate::interpreter::run_to_completion;

fn main() {
    let prog: Vec<isize> = include_str!("input")
        .split(",")
        .filter_map(|x| x.parse::<isize>().ok())
        .collect();

    let part1 = run_to_completion(&prog, &[1]);
    println!("part1: {:?}", part1);

    let part2 = run_to_completion(&prog, &[2]);
    println!("part2: {:?}", part2);
}
