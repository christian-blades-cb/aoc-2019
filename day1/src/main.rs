fn main() {
    let input = include_str!("input");

    let nums: Vec<usize> = input
        .lines()
        .filter_map(|x| str::parse::<usize>(x).ok())
        .collect();

    println!("part1: {}", part1(&nums));
    println!("part2: {}", part2(&nums));
}

fn part1(xs: &[usize]) -> usize {
    xs.iter().map(|x| x / 3 - 2).sum()
}

fn part2(xs: &[usize]) -> usize {
    xs.iter().map(|x| fuel_comp(x / 3 - 2)).sum()
}

fn fuel_comp(fuel: usize) -> usize {
    if fuel / 3 < 3 {
        return fuel;
    }

    fuel_comp(fuel / 3 - 2) + fuel
}
