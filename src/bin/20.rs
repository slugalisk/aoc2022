use std::{collections::VecDeque, fs};

fn decrypt(n: i32, coords: Vec<i64>) -> i64 {
    let mut ordered = VecDeque::from_iter(coords.into_iter().enumerate());
    let l = ordered.len();

    for _ in 0..n {
        for i in 0..l {
            let j = ordered.iter().position(|(t, _)| *t == i).unwrap();
            ordered.rotate_left(j);
            let v = ordered.pop_front().unwrap();
            if v.1 > 0 {
                ordered.rotate_left(v.1 as usize % (l - 1));
            } else {
                ordered.rotate_right(-v.1 as usize % (l - 1));
            }
            ordered.push_front(v);
        }
    }

    let i = ordered.iter().position(|(_, v)| *v == 0).unwrap();
    ordered.rotate_left(i);
    ordered[1000 % l].1 + ordered[2000 % l].1 + ordered[3000 % l].1
}

fn main() {
    let input = fs::read_to_string("./inputs/20.txt").unwrap();
    let coords = input
        .split('\n')
        .filter_map(|t| t.parse::<i64>().ok())
        .collect::<Vec<_>>();

    println!("part1: {:?}", decrypt(1, coords.clone()));
    println!(
        "part2: {:?}",
        decrypt(10, coords.iter().map(|t| (*t) * 811589153).collect())
    );
}
