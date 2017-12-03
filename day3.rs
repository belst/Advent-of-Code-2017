use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


fn distance((x, y): (i64, i64)) -> u64 {
    x.abs() as u64 + y.abs() as u64
}

/// Get coordinates in grid, (0,0) at center (1)
fn coordinates(n: i64) -> (i64, i64) {
    let k = (((n as f64).sqrt() - 1f64) / 2f64).ceil() as i64;
    let mut t = 2 * k + 1;
    let mut m = t.pow(2) as i64;

    t -= 1;
    if n >= m - t {
        return (k - m + n, -k);
    }

    m -= t;
    if n >= m - t {
        return (-k, -k + m - n);
    }

    m -= t;
    if n >= m - t {
        return (-k + m - n, k);
    }

    (k, k - m + n + t)
}

fn lookup(n: u64) -> u64 {
    let file = File::open("day3part2table.txt").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.starts_with('#') && !l.is_empty())
        .map(|l| {
            l.split_whitespace()
                .nth(1)
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap()
        })
        .skip_while(|&s| s <= n)
        .next()
        .unwrap()
}


fn main() {
    let input = std::env::args()
        .nth(1)
        .and_then(|n| n.parse::<i64>().ok())
        .unwrap();

    println!("Part1: {}", distance(coordinates(input)));
    println!("Part2: {}", lookup(input as u64));

}
