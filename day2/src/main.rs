fn main() {
    let input = include_str!("input");

    let nums: Vec<usize> = input
        .split(",")
        .filter_map(|x| x.trim().parse::<usize>().ok())
        .collect();

    // part1(&vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
    let target = 19690720;
    println!("part1: {}", run(&nums, 12, 2));
    for noun in 0..=99 {
        for verb in 0..=99 {
            if run(&nums, noun, verb) == target {
                println!("part2: {} (n:{} v:{})", 100 * noun + verb, noun, verb);
            }
        }
    }
} // 93042 too high

fn run(xs: &[usize], noun: usize, verb: usize) -> usize {
    let mut pc = 0;
    let mut prog: Vec<usize> = xs.to_vec();
    prog[1] = noun;
    prog[2] = verb;

    loop {
        let op = prog[pc];
        // println!("{:?}", prog);
        match op {
            1 => {
                // sum
                let left_pos = prog[pc + 1];
                let right_pos = prog[pc + 2];
                let dest_pos = prog[pc + 3];

                let left = prog[left_pos];
                let right = prog[right_pos];

                let acc = left + right;

                // println!(
                //     "[{}]\t {} {} {} {} => {} + {} = {}",
                //     pc, op, left_pos, right_pos, dest_pos, left, right, acc
                // );

                prog[dest_pos] = acc;

                pc += 4;
            }
            2 => {
                // product
                let left_pos = prog[pc + 1];
                let right_pos = prog[pc + 2];
                let dest_pos = prog[pc + 3];

                let left = prog[left_pos];
                let right = prog[right_pos];

                let acc = left * right;

                // println!(
                //     "[{}]\t {} {} {} {} => {} + {} = {}",
                //     pc, op, left_pos, right_pos, dest_pos, left, right, acc
                // );

                prog[dest_pos] = acc;

                pc += 4;
            }
            99 => break,
            _ => unreachable!(),
        }
    }

    prog[0]
}
// 406198 too low
