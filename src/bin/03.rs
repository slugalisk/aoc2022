use itertools::Itertools;
use std::collections::HashMap;
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

    let mut found: HashMap<i32, bool> = HashMap::new();
    for s in sacks.iter() {
        found.clear();
        let mid = s.len() / 2;
        for w in s.iter().take(mid) {
            found.insert(*w, true);
        }
        for w in s.iter().skip(mid) {
            if found.contains_key(w) {
                part1_sum += w;
                break;
            }
        }
    }

    println!("part1: {:?}", part1_sum);

    let mut part2_sum = 0;

    for mut g in &sacks.iter().chunks(3) {
        let s0 = g.next().unwrap();
        let s1 = g.next().unwrap();
        let s2 = g.next().unwrap();
        let v = s0
            .iter()
            .find(|w| s1.contains(*w) && s2.contains(*w))
            .unwrap();

        part2_sum += v;
    }

    println!("part2: {:?}", part2_sum);
}
