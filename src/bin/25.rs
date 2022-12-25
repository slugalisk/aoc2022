use std::fs;

fn parse_snafu(s: &str) -> usize {
    s.chars().fold(0, |n, d| {
        n * 5 + "=-012".chars().position(|x| x == d).unwrap() - 2
    })
}

fn format_snafu(n: usize) -> String {
    if n == 0 {
        "".to_string()
    } else {
        format_snafu((n + 2) / 5) + ["0", "1", "2", "=", "-"][n % 5]
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/25.txt").unwrap();
    let sum = input.trim().split('\n').fold(0, |n, l| n + parse_snafu(l));
    println!("part1: {:?}", format_snafu(sum));
}
