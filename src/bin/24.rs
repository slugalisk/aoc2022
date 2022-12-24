use std::collections::HashSet;
use std::fs;

fn update_map(map: &mut [Vec<u8>]) {
    let h = map.len();
    let w = map[0].len();

    for y in 1..h - 1 {
        for x in 1..w - 1 {
            macro_rules! update {
                ($i:tt, $p:tt, $v:tt) => {
                    if map[y][x] & $i != 0 {
                        *map.get_mut(($p.1 - 1).rem_euclid(h - 2) + 1)
                            .unwrap()
                            .get_mut(($p.0 - 1).rem_euclid(w - 2) + 1)
                            .unwrap() |= $v << 4;
                    }
                };
            }

            update!(1, (x, y + h - 3), 1);
            update!(2, (x, y + 1), 2);
            update!(4, (x + w - 3, y), 4);
            update!(8, (x + 1, y), 8);
        }
    }
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            *map.get_mut(y).unwrap().get_mut(x).unwrap() >>= 4;
        }
    }
}

fn run(map: &Vec<Vec<u8>>, waypoints: &[(i32, i32)]) -> Option<i32> {
    let mut map = map.clone();
    let mut queue = HashSet::from([(1, waypoints[0])]);
    for t in 0.. {
        update_map(&mut map);
        let mut next_queue = HashSet::new();
        for (mut wi, p) in queue {
            if p == waypoints[wi] {
                wi += 1;
                if wi == waypoints.len() {
                    return Some(t);
                }
            }

            for (dx, dy) in [(0, 0), (0, -1), (0, 1), (-1, 0), (1, 0)] {
                let np = (p.0 + dx, p.1 + dy);
                if 0.eq(map
                    .get(np.1 as usize)
                    .and_then(|t| t.get(np.0 as usize))
                    .unwrap_or(&1))
                {
                    next_queue.insert((wi, np));
                }
            }
        }
        queue = next_queue;
    }
    None
}

fn main() {
    let input = fs::read_to_string("./inputs/24.txt").unwrap();
    let map = input
        .trim()
        .split('\n')
        .map(|t| {
            t.chars()
                .map(|c| match c {
                    '.' => 0,
                    '^' => 1,
                    'v' => 2,
                    '<' => 4,
                    '>' => 8,
                    '#' => 16,
                    _ => unreachable!(),
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();

    let start = (1, 0);
    let end = (map[0].len() as i32 - 2, map.len() as i32 - 1);

    println!("part1: {:?}", run(&map, &[start, end]));
    println!("part2: {:?}", run(&map, &[start, end, start, end]));
}
