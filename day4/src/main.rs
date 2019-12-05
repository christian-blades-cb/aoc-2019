fn main() {
    let input = include_str!("input");
    let (begin, end) = {
        let mut range = input.split("-").map(str::parse::<isize>);
        (
            range.next().unwrap().unwrap(),
            range.next().unwrap().unwrap(),
        )
    };

    dbg!(begin, end);
    println!("part1 {:?}", part1(begin, end));
    println!("part2 {:?}", part2(begin, end));
}

fn part1(begin: isize, end: isize) -> usize {
    filter_range(begin, end).len()
}

fn filter_range(begin: isize, end: isize) -> Vec<isize> {
    (begin..=end)
        .filter_map(|x| {
            let digits = format!("{}", &x).into_bytes();

            let diffs = digits
                .iter()
                .cloned()
                .map(|x| x as i16)
                .scan(Option::<i16>::None, {
                    |prev, x| {
                        let diff = prev.map(|p| x - p).unwrap_or(x);
                        *prev = Some(x);
                        Some(diff)
                    }
                });

            let (double, decreases) =
                diffs.fold((false, false), |(dbl, dec), diff| match (dec, diff) {
                    (true, _) => (dbl, dec),
                    _ if diff < 0 => (dbl, true),
                    _ if diff == 0 => (true, dec),
                    _ => (dbl, dec),
                });

            if double && !decreases {
                Some(x)
            } else {
                None
            }
        })
        .collect()
}

fn part2(begin: isize, end: isize) -> usize {
    filter_range(begin, end)
        .iter()
        .filter(|x| part_2_validation(x))
        .count()
}

fn part_2_validation(x: &isize) -> bool {
    let bleg = format!("{}", x).into_bytes();
    let mut chars = bleg.iter();
    let mut count = 1;
    let mut prev = chars.next().unwrap();

    let mut repeats = Vec::new();

    for x in chars {
        if prev == x {
            count += 1;
        } else {
            repeats.push((prev, count));
            prev = x;
            count = 1;
        }
    }

    repeats.push((prev, count));

    repeats.iter().any(|(_, count)| *count == 2)
}
