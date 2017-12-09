use std::io::{self, Read};

enum State {
    Def,
    Garbage,
    Ignore
}

fn main() {
    let mut input = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut input).unwrap();

    // keep track of nesting depth
    let mut depth = 0;
    let mut score = 0;
    // Garbage count (Part 2)
    let mut gc = 0;
    
    let mut s = State::Def;

    for c in input.chars() {
        use State::*;
        match s {
            Def => match c {
                '<' => s = Garbage,
                '{' => depth += 1,
                '}' => {
                    score += depth;
                    depth -= 1;
                }
                _ => {},
            },
            Garbage => match c {
                '!' => s = Ignore,
                '>' => s = Def,
                _ => gc += 1,
            },
            Ignore => s = Garbage,
        };
    };

    println!("Part1: {}", score);
    println!("Part2: {}", gc);
}