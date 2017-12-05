use std::io::{self, BufRead};

fn part1(input: &[isize]) -> usize {
    let mut steps = 0;
    let mut input = input.to_vec();
    let mut index: isize = 0;

    while let Some(e) = input.get_mut(index as usize) {
        index += *e;
        *e += 1;
        steps += 1;
    }
    steps
}

fn part2(input: &[isize]) -> usize {
    let mut steps = 0;
    let mut input = input.to_vec();
    let mut index: isize = 0;

    while let Some(e) = input.get_mut(index as usize) {
        index += *e;
        *e += if 2 < *e { -1 } else { 1 };
        steps += 1;
    }
    steps
}

fn main() {
    let stdin = io::stdin();
    let lock = stdin.lock();
    let input: Vec<_> = lock.lines()
        .map(|l| l.unwrap().parse::<isize>().unwrap())
        .collect();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
