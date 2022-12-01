use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/01.txt").unwrap();
    let mut elves: Vec<i32> = input
        .split("\n\n")
        .into_iter()
        .map(|c| {
            c.split('\n')
                .filter(|v| !v.is_empty())
                .map(|v| v.parse::<i32>().unwrap())
                .sum()
        })
        .collect();
    elves.sort_by(|a, b| b.partial_cmp(a).unwrap());

    println!("part1: {:?}", elves[0]);
    println!("part2: {:?}", elves.iter().take(3).sum::<i32>());
}
