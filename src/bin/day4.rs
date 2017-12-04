extern crate itertools;

use std::io;
use std::io::prelude::*;
use itertools::Itertools;

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|l| {
            let words = l.split_whitespace();
            if words.clone().unique().count() == words.count() {
                Some(())
            } else {
                None
            }
        })
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter_map(|l| {
            let words = l.split_whitespace()
                .map(|w| {
                    let mut v = w.as_bytes().to_owned(); // ascii only
                    v.sort();
                    v
                })
                .collect::<Vec<_>>();
            if words.iter().unique().count() == words.iter().count() {
                Some(())
            } else {
                None
            }
        })
        .count()
}

fn main() {
    let mut input = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut input).unwrap();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}
