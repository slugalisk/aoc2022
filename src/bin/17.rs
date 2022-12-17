use std::cmp::Ordering::{Equal, Greater, Less};
use std::fs;

fn shift(l: u8, x: i32) -> u8 {
    match 0.cmp(&x) {
        Greater => l << -x,
        Less => l >> x,
        Equal => l,
    }
}

fn measure_rock_fall(shapes: &[Vec<u8>], wind: &[i32], n: usize) -> (usize, usize) {
    let mut chamber = vec![];
    let mut peak = 0;
    let mut t = 0;
    let mut prev_tm = 0;
    let mut seq_start = 0;
    for i in 0..n {
        let p = shapes[i % shapes.len()].clone();
        let mut y = peak + 2 + p.len();
        let mut x = 0;

        while y >= chamber.len() {
            chamber.push(0);
        }

        loop {
            let dx = wind[t % wind.len()];
            t += 1;

            let mut blocked = false;
            for i in 0..p.len() {
                let pi = shift(p[i], x + dx);
                if pi.count_ones() < p[i].count_ones() || pi >= 1 << 7 || pi & chamber[y - i] != 0 {
                    blocked = true;
                    break;
                }
            }
            if !blocked {
                x += dx;
            }

            let mut blocked = false;
            for i in 0..p.len() {
                let pi = shift(p[i], x);
                if y - i == 0 || pi & chamber[y - i - 1] != 0 {
                    blocked = true;
                    break;
                }
            }
            if blocked {
                for i in 0..p.len() {
                    *(chamber.get_mut(y - i).unwrap()) |= shift(p[i], x);
                }
                peak = peak.max(y + 1);
                break;
            }
            y -= 1;
        }

        let tm = t % wind.len();
        if seq_start == 0 && tm < prev_tm {
            seq_start = i;
        }
        prev_tm = tm;
    }
    (peak, seq_start)
}

fn main() {
    let input = fs::read_to_string("./inputs/17.txt").unwrap();
    let wind = input
        .chars()
        .filter_map(|c| match c {
            '<' => Some(-1),
            '>' => Some(1),
            _ => None,
        })
        .collect::<Vec<_>>();

    let shapes: Vec<Vec<u8>> = vec![
        vec![0b11110],
        vec![0b1000, 0b11100, 0b1000],
        vec![0b100, 0b100, 0b11100],
        vec![0b10000, 0b10000, 0b10000, 0b10000],
        vec![0b11000, 0b11000],
    ];

    let (part1_height, _) = measure_rock_fall(&shapes, &wind, 2022);
    println!("part1: {}", part1_height);

    let (_, seq_start) = measure_rock_fall(&shapes, &wind, wind.len());
    let seq_len = seq_start + 1;
    let (d0, _) = measure_rock_fall(&shapes, &wind, seq_start);
    let (d1, _) = measure_rock_fall(&shapes, &wind, seq_start + seq_len);
    let rock_count = 1000000000000;
    let n0 = (rock_count - seq_start) / seq_len;
    let n1 = (rock_count - seq_start) % seq_len;
    let (d2, _) = measure_rock_fall(&shapes, &wind, seq_start + n1);
    println!("part2: {}", n0 * (d1 - d0) + d2);
}
