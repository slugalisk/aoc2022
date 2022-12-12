use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

fn bfs_len(
    grid: &[Vec<i32>],
    height: i32,
    width: i32,
    start: (i32, i32),
    end: (i32, i32),
) -> Option<i32> {
    let mut queue = VecDeque::from([(0, start)]);
    let mut visited = HashSet::from([start]);
    while let Some((p, (x, y))) = queue.pop_front() {
        if (x, y) == end {
            return Some(p);
        }
        let e = grid[y as usize][x as usize];
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = (x + dx).clamp(0, width - 1);
            let ny = (y + dy).clamp(0, height - 1);
            let ne = grid[ny as usize][nx as usize];
            if visited.contains(&(nx, ny)) || ne - e > 1 {
                continue;
            }
            visited.insert((nx, ny));
            queue.push_back((p + 1, (nx, ny)));
        }
    }
    None
}

fn main() {
    let input = fs::read_to_string("./inputs/12.txt").unwrap();
    let grid = input
        .trim()
        .split('\n')
        .map(|t| {
            t.chars()
                .map(|t| match t {
                    'S' => 0,
                    'E' => 26,
                    _ => t as i32 - 97,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let start_index = input.find('S').unwrap() as i32;
    let end_index = input.find('E').unwrap() as i32;
    let start = (start_index % (width + 1), start_index / (width + 1));
    let end = (end_index % (width + 1), end_index / (width + 1));

    println!("part1: {:?}", bfs_len(&grid, height, width, start, end));

    let mut min_len = std::i32::MAX;
    for x in 0..width {
        for y in 0..height {
            if grid[y as usize][x as usize] != 0 {
                continue;
            }
            if let Some(p) = bfs_len(&grid, height, width, (x, y), end) {
                min_len = min_len.min(p);
            }
        }
    }
    println!("part2: {}", min_len);
}
