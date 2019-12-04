#[derive(Debug, PartialEq)]
enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
struct Segment {
    dir: Direction,
    start: Coord,
    end: Coord,
}

type Coord = (isize, isize);

impl Segment {
    fn intersection(&self, other: &Segment) -> Option<Coord> {
        if self.dir == other.dir {
            return None;
        }

        match self.dir {
            Direction::Horizontal => Self::helper(self, other),
            Direction::Vertical => Self::helper(other, self),
        }
    }

    fn helper(horiz: &Self, vert: &Self) -> Option<Coord> {
        let (x1, y_plane) = horiz.start;
        let (x2, _y) = horiz.end;
        let (x_start, x_end) = if x1 < x2 { (x1, x2) } else { (x2, x1) };

        let (x_plane, y1) = vert.start;
        let (_x, y2) = vert.end;
        let (y_start, y_end) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

        if x_start <= x_plane && x_end >= x_plane && y_start <= y_plane && y_end >= y_plane {
            Some((x_plane, y_plane))
        } else {
            None
        }
    }
}

fn main() {
    let input = include_str!("input");

    let mut lines: Vec<Vec<Segment>> = input.lines().map(parse_line).collect();
    let set2 = lines.pop().unwrap();
    let set1 = lines.pop().unwrap();

    println!("part1: {:?}", part1(&set1, &set2));
}

fn part1(a: &[Segment], b: &[Segment]) -> Option<isize> {
    let mut intersections = std::collections::HashSet::new();

    for x in a {
        for y in b {
            if let Some(crossing) = x.intersection(&y) {
                intersections.insert(crossing);
            }
        }
    }

    intersections.iter().map(|(x, y)| x.abs() + y.abs()).min()
}

fn parse_line(x: &str) -> Vec<Segment> {
    x.split(",")
        .fold(((0, 0), Vec::new()), |(prev, mut acc), y| {
            if y.len() < 2 {
                return (prev, acc);
            }

            let (dir, rest) = y.split_at(1);
            let steps = rest.parse::<isize>().unwrap();

            let (dir, end) = match dir {
                "R" => {
                    let end = (prev.0 + steps, prev.1);
                    (Direction::Horizontal, end)
                }
                "L" => {
                    let end = (prev.0 - steps, prev.1);
                    (Direction::Horizontal, end)
                }
                "U" => {
                    let end = (prev.0, prev.1 + steps);
                    (Direction::Vertical, end)
                }
                "D" => {
                    let end = (prev.0, prev.1 - steps);
                    (Direction::Vertical, end)
                }
                _ => unreachable!(),
            };

            acc.push(Segment {
                dir,
                end: Clone::clone(&end),
                start: Clone::clone(&prev),
            });

            (end, acc)
        })
        .1
}
