fn main() {
    let input: Vec<u8> = include_bytes!("input").iter().map(|x| x - 48).collect();
    println!("part1: {}", part1(&input));
    println!("part2:");
    part2(&input);
}

fn part1(input: &[u8]) -> usize {
    use itertools::Itertools;

    // 25 x 6

    input
        .iter()
        .chunks(25 * 6)
        .into_iter()
        .map(|chunk| {
            chunk.fold((0, 0, 0), |(z, o, t), x| match x {
                0 => (z + 1, o, t),
                1 => (z, o + 1, t),
                2 => (z, o, t + 1),
                _ => (z, o, t),
            })
        })
        .enumerate()
        // .inspect(|x| {
        //     dbg!(x);
        // })
        .min_by_key(|(_i, (zeros, _, _))| *zeros)
        .map(|(_i, (_z, o, t))| o * t)
        .unwrap()
}

fn part2(input: &[u8]) {
    use std::io::prelude::*;

    let img = input.chunks(25 * 6).fold(vec![2; 25 * 6], |acc, layer| {
        itertools::zip(acc, layer)
            .map(|(a, b)| match a {
                2 => *b,
                _ => a,
            })
            .collect()
    });

    let mut o = std::io::stdout();
    for (i, x) in img.iter().enumerate() {
        if i % 25 == 0 {
            write!(o, "\n").unwrap();
        }

        write!(
            o,
            "{}",
            match x {
                0 => " ",
                1 => "█",
                2 => ".",
                _ => unreachable!(),
            }
        )
        .unwrap();
    }
}
