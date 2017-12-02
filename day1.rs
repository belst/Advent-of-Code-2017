fn solve(input: &str, skip: usize) -> u32 {
    input.chars()
        .zip(input.chars().cycle().skip(skip))
        .filter_map(|(x, y)| if x == y { x.to_digit(10) } else { None })
        .sum()
}

fn solve1(input: &str) -> u32 {
    solve(input, 1)
}

fn solve2(input: &str) -> u32 {
    let len = input.len();
    solve(input, len / 2)
}

fn main() {
    let input = std::env::args().nth(1).unwrap();
    println!("Captcha1: {}", solve1(&input));
    println!("Captcha2: {}", solve2(&input));
}

