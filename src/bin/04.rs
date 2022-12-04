use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/04.txt").unwrap();
    let assns = input
        .split('\n')
        .filter_map(|c| {
            if c.is_empty() {
                None
            } else {
                let mut a = c.split(',').map(|r| {
                    let mut l = r.split('-').map(|v| v.parse::<i32>().unwrap());
                    (l.next().unwrap(), l.next().unwrap())
                });
                Some((a.next().unwrap(), a.next().unwrap()))
            }
        })
        .collect::<Vec<_>>();

    let mut part1_count = 0;

    for a in assns.iter() {
        if (a.0 .0 >= a.1 .0 && a.0 .1 <= a.1 .1) || (a.1 .0 >= a.0 .0 && a.1 .1 <= a.0 .1) {
            part1_count += 1;
        }
    }

    println!("part1: {:?}", part1_count);

    let mut part2_count = 0;

    for a in assns.iter() {
        if a.0 .0 <= a.1 .1 && a.0 .1 >= a.1 .0 {
            part2_count += 1;
        }
    }

    println!("part2: {:?}", part2_count);
}
