use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/03.txt").unwrap();
    let sacks: Vec<Vec<i32>> = input
        .split('\n')
        .filter_map(|c| {
            if c.is_empty() {
                None
            } else {
                Some(
                    c.chars()
                        .map(|t| {
                            if t >= 'a' {
                                t as i32 - 96
                            } else {
                                t as i32 - 38
                            }
                        })
                        .collect(),
                )
            }
        })
        .collect();

    let mut part1_sum = 0;

    for s in sacks.iter() {
        let mid = s.len() / 2;
        let a: HashSet<&i32> = HashSet::from_iter(s.iter().take(mid));
        let b: HashSet<&i32> = HashSet::from_iter(s.iter().skip(mid));
        let v = a.intersection(&b).next().unwrap();
        part1_sum += **v;
    }

    println!("part1: {:?}", part1_sum);

    let mut part2_sum = 0;

    for g in &sacks.iter().chunks(3) {
        let mut sacks: Vec<HashSet<&i32>> = g.map(|s| HashSet::from_iter(s.iter())).collect();
        let mut v = sacks.pop().unwrap();
        v.retain(|w| sacks[0].contains(*w) && sacks[1].contains(*w));
        part2_sum += *(v.iter().next().unwrap());
    }

    println!("part2: {:?}", part2_sum);
}
