use std::fs;

use itertools::Itertools;

fn mark_visited(pos: (i32, i32), visited: &mut [Vec<i32>]) {
    let v = visited
        .get_mut(pos.1 as usize)
        .unwrap()
        .get_mut(pos.0 as usize)
        .unwrap();
    *v = 1;
}

fn count_visited(visited: &[Vec<i32>]) -> i32 {
    visited.iter().map(|t| t.iter().sum::<i32>()).sum::<i32>()
}

fn move_rope(d: &str, r: &mut [(i32, i32)]) {
    let h = r.get_mut(0).unwrap();
    match d {
        "L" => h.0 -= 1,
        "R" => h.0 += 1,
        "U" => h.1 -= 1,
        "D" => h.1 += 1,
        &_ => println!("unrecognized direction {}", d),
    }

    for i in 1..r.len() {
        let h = r[i - 1];
        let t = r.get_mut(i).unwrap();

        let dx = h.0 - t.0;
        let dy = h.1 - t.1;

        if dx.abs() + dy.abs() > 2 {
            t.0 += dx.clamp(-1, 1);
            t.1 += dy.clamp(-1, 1);
        } else if dx.abs() > 1 {
            t.0 += dx.clamp(-1, 1);
        } else if dy.abs() > 1 {
            t.1 += dy.clamp(-1, 1);
        }
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/09.txt").unwrap();

    let moves = input.split('\n').filter_map(|t| {
        let mut p = t.split(' ');
        if let (Some(d), Some(n)) = (p.next(), p.next()) {
            Some((d, n.parse::<i32>().unwrap()))
        } else {
            None
        }
    });

    let mut part1_rope = vec![(500, 500); 2];
    let mut part2_rope = vec![(500, 500); 10];
    let mut part1_visited = vec![vec![0; 1000]; 1000];
    let mut part2_visited = vec![vec![0; 1000]; 1000];
    mark_visited(part1_rope[1], &mut part1_visited);
    mark_visited(part2_rope[9], &mut part2_visited);

    for (d, n) in moves {
        for _i in 0..n {
            move_rope(d, &mut part1_rope);
            mark_visited(part1_rope[1], &mut part1_visited);
            move_rope(d, &mut part2_rope);
            mark_visited(part2_rope[9], &mut part2_visited);
        }
    }

    println!("part1 {:?}", count_visited(&part1_visited));
    println!("part2 {:?}", count_visited(&part2_visited));
}
