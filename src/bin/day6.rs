use std::io::Read;

// Finds maximum (if multiple, first)
// max_by finds last
fn find_max(arr: &[usize]) -> (usize, usize) {
    arr.iter()
        .enumerate()
        .fold((0, 0), |(index, max), (curr_index, &val)| {
            if val > max {
                (curr_index, val)
            } else {
                (index, max)
            }
        })
}

fn iterate(arr: &mut [usize]) {
    let (mut i, mut max) = find_max(arr);
    let l = arr.len();
    arr[i] = 0; // clear
    i += 1; // inc index

    while max > 0 {
        arr[i % l] += 1;
        max -= 1;
        i += 1;
    }
}

fn main() {
    let mut seen: Vec<Vec<usize>> = Vec::new();

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut input: Vec<usize> = input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let mut iterations = 0;

    while !seen.contains(&input) {
        seen.push(input.to_vec());
        iterate(&mut input);
        iterations += 1;
    }
    println!("Part1: {}", iterations);

    let first = seen.iter().position(|s| &input == s).unwrap();
    println!("Part2: {}", seen.len() - first);
}
