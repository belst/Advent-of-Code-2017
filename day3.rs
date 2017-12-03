
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


fn main() {
    let input = std::env::args()
        .nth(1)
        .and_then(|n| n.parse::<i64>().ok())
        .unwrap();

    println!("Part1: {}", distance(coordinates(input)));

}
