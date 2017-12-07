use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug, Clone, Default)]
struct Program {
    name: String,
    weight: usize,
    children: Vec<Program>,
}

#[derive(Debug, Clone, Default)]
struct ParseProgram {
    name: String,
    weight: usize,
    children: Vec<String>,
}
// Copying and allocating left right and center. cba borrowing
fn parse(input: &str) -> Program {
    let mut map: HashMap<String, ParseProgram> = HashMap::new();

    for mut l in input.lines().map(|l| l.split_whitespace()) {
        let name = l.next().unwrap();
        let weight = l.next().unwrap();
        // strip parens
        let weight: usize = weight[1..weight.len() - 1].parse().unwrap();
        let children: Vec<_> = match l.next() { // ->
            None => vec![],
            Some(_) => l.map(|n| n.trim_right_matches(',').into()).collect(),
        };
        map.insert(
            name.into(),
            ParseProgram {
                name: name.into(),
                weight,
                children,
            },
        );
    }

    let children = map.values().flat_map(|p| &p.children).collect::<Vec<_>>();
    let root = map.values().find(|p| !children.contains(&&p.name)).unwrap();

    build_program(&map, &root.name)
}

fn build_program(map: &HashMap<String, ParseProgram>, name: &str) -> Program {
    Program {
        name: name.into(),
        weight: map[name].weight,
        children: map[name]
            .children
            .iter()
            .map(|p| build_program(map, p))
            .collect(),
    }
}

fn find_imbalance(prog: &Program) -> &Vec<Program> {
    if prog.children.iter().all(|c| balanced(&c.children)) {
        // found imbalanced node
        return &prog.children;
    }
    let new = prog.children
        .iter()
        .find(|c| !balanced(&c.children))
        .unwrap(); // at least 1 exists
    find_imbalance(new)
}

// calculate weights a thousand times because ¯\_(ツ)_/¯
fn balanced(children: &[Program]) -> bool {
    if children.is_empty() || children.len() == 1 {
        return true;
    }
    let mut b = true;
    for c in children {
        b = b && weight(c) == weight(&children[0]);
    }
    b
}

fn weight(prog: &Program) -> usize {
    prog.weight + prog.children.iter().fold(0, |acc, p| acc + weight(p))
}

fn get_diff(progs: &[Program]) -> usize {
    let avg: f64 = progs.iter().map(weight).sum::<usize>() as f64 / progs.len() as f64;

    let odd = progs
        .iter()
        .max_by(|l, r| {
            (weight(l) as f64 - avg)
                .abs()
                .partial_cmp(&(weight(r) as f64 - avg).abs())
                .unwrap()
        })
        .unwrap();
    let norm = progs.iter().find(|p| p.name != odd.name).unwrap();
    let diff = weight(odd) - weight(norm);
    odd.weight - diff
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let program = parse(&input);
    println!("Part1: {}", program.name);
    println!("Part2: {}", get_diff(find_imbalance(&program)));
}
