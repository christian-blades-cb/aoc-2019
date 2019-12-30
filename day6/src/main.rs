use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Orbit {
    name: String,
    children: Vec<Self>,
}

fn main() {
    let input = include_str!("input");
    let pairs: Vec<(&str, &str)> = input
        .lines()
        .filter_map(|x| {
            let mut p = x.trim().split(")");
            p.next().and_then(|j| p.next().map(|k| (j, k)))
        })
        .collect();

    let roots = find_roots(&pairs);
    // dbg!(&roots);
    let primary_root = roots.get(0).cloned().unwrap();

    let mut hm: HashMap<&str, Vec<String>> = HashMap::new();
    for (p, c) in pairs {
        hm.entry(p)
            .and_modify(|e| e.push(c.to_string()))
            .or_insert(vec![c.to_string()]);
    }

    let tree = build_tree(primary_root, &mut hm);
    println!("part1: {}", part1(&tree));

    println!("part2: {}", part2(&tree));
}

fn part1(tree: &Orbit) -> usize {
    count_orbits(&tree, 0)
}

fn part2(tree: &Orbit) -> usize {
    // d_r = depth of common root - 3
    // d_x = depth of self - 7
    // d_y = depth of other - 5
    // (d_x - d_r - 1) + (d_y - d_r - 1)
    // (7 - 3 - 1) + (5 - 3 - 1)

    let you_parents = parents_of("YOU", &tree, &Vec::new()).unwrap();
    // dbg!(&you_parents);
    let san_parents = parents_of("SAN", &tree, &Vec::new()).unwrap();
    // dbg!(&san_parents);

    let you_ancestor_count = you_parents.len();
    let san_ancestor_count = san_parents.len();

    let you_parents: HashSet<(usize, String)> = you_parents.iter().cloned().enumerate().collect();
    let san_parents: HashSet<(usize, String)> = san_parents.iter().cloned().enumerate().collect();
    let common_ancestor = you_parents
        .intersection(&san_parents)
        .max_by(|(depth_x, _), (depth_y, _)| depth_x.cmp(&depth_y))
        .unwrap();

    // dbg!(&common_ancestor);

    let (common_depth, _) = common_ancestor;

    (you_ancestor_count - 1 - common_depth) + (san_ancestor_count - 1 - common_depth)
}

fn find_roots<'a>(xs: &[(&'a str, &'a str)]) -> Vec<&'a str> {
    let mut parents = HashSet::new();
    let mut children = HashSet::new();

    for (p, c) in xs.iter().cloned() {
        parents.insert(p);
        children.insert(c);
    }

    parents.difference(&children).cloned().collect()
}

fn build_tree(root: &str, mut mapping: &mut HashMap<&str, Vec<String>>) -> Orbit {
    let children = mapping.remove(root);

    Orbit {
        name: root.to_string(),
        children: children
            .unwrap_or(Vec::new())
            .iter()
            .map(|c| build_tree(c, &mut mapping))
            .collect(),
    }
}

fn count_orbits(node: &Orbit, depth: usize) -> usize {
    node.children
        .iter()
        .map(|c| count_orbits(c, depth + 1))
        .sum::<usize>()
        + depth
}

fn parents_of(dest: &str, tree: &Orbit, ancestors: &[&str]) -> Option<Vec<String>> {
    if tree.name == dest {
        return Some(ancestors.iter().map(|s| s.to_string()).collect());
    }

    let mut ancestors = ancestors.to_vec();
    ancestors.push(&tree.name);

    for n in tree.children.iter() {
        let parents = parents_of(dest, n, &ancestors);
        if parents.is_some() {
            return parents;
        }
    }

    None
}
