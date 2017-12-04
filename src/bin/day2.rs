use std::io::{self, Read};
use std::u64;
use std::cmp::{max, min};

fn problem1(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|s| s.parse::<u64>().ok())
                // find min and max in one iteration
                .fold((u64::MAX, u64::MIN), |(min, max), c| if c > max {
                    (min, c)
                } else if c < min {
                    (c, max)
                } else {
                    (min, max)
                })
        })
        .map(|(min, max)| max - min)
        .sum()
}

fn problem2(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            find_pair(l.split_whitespace().filter_map(|s| s.parse::<u64>().ok()))
        })
        .map(|(a, b)| a / b)
        .sum()
}

// ugly. THIS MAKES ME SICK
fn find_pair<I: Iterator<Item = u64>>(it: I) -> (u64, u64) {
    let it2 = it.collect::<Vec<_>>();
    let it = it2.clone();
    for (i, a) in it2.iter().enumerate() {
        for b in it.clone().iter().skip(i + 1) {
            if max(a, &b) % min(a, &b) == 0 {
                return (max(*a, *b), min(*a, *b));
            }
        }
    }
    panic!("Invalid input. no pair found");
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    println!("Part 1: {}", problem1(&buf));
    println!("Part 2: {}", problem2(&buf));
}
