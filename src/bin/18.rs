use std::cmp::PartialEq;
use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Clone, Copy, PartialEq)]
enum Elem {
    Air,
    Water,
    Rock,
}

fn main() {
    let input = fs::read_to_string("./inputs/18.txt").unwrap();

    let mut size = 0;

    let coords = input
        .split('\n')
        .filter(|t| !t.is_empty())
        .map(|t| {
            let mut p = t.split(',').map(|t| t.parse::<usize>().unwrap());
            let x = p.next().unwrap();
            let y = p.next().unwrap();
            let z = p.next().unwrap();
            size = size.max(x).max(y).max(z);
            (x, y, z)
        })
        .collect::<Vec<_>>();

    size += 3;
    let mut space = vec![vec![vec![Elem::Air; size]; size]; size];

    let fill_dirs = [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];

    for (x, y, z) in coords.iter() {
        *(space
            .get_mut(*x + 1)
            .unwrap()
            .get_mut(*y + 1)
            .unwrap()
            .get_mut(*z + 1)
            .unwrap()) = Elem::Rock;
    }

    let mut part1_n = 0;
    for (x, dy) in space.iter().enumerate() {
        for (y, dz) in dy.iter().enumerate() {
            for (z, _) in dz.iter().enumerate().filter(|(_, &v)| v == Elem::Rock) {
                for (dx, dy, dz) in fill_dirs {
                    let tx = x as i32 + dx;
                    let ty = y as i32 + dy;
                    let tz = z as i32 + dz;
                    if space[tx as usize][ty as usize][tz as usize] != Elem::Rock {
                        part1_n += 1;
                    }
                }
            }
        }
    }

    println!("part1: {:?}", part1_n);

    let mut queue = VecDeque::from([(0, 0, 0)]);
    let mut visited = HashSet::from([(0, 0, 0)]);
    while let Some((x, y, z)) = queue.pop_front() {
        *(space
            .get_mut(x)
            .unwrap()
            .get_mut(y)
            .unwrap()
            .get_mut(z)
            .unwrap()) = Elem::Water;

        for (dx, dy, dz) in fill_dirs {
            let tx = x as i32 + dx;
            let ty = y as i32 + dy;
            let tz = z as i32 + dz;
            if (0..size as i32).contains(&tx)
                && (0..size as i32).contains(&ty)
                && (0..size as i32).contains(&tz)
                && space[tx as usize][ty as usize][tz as usize] == Elem::Air
                && !visited.contains(&(tx, ty, tz))
            {
                visited.insert((tx as i32, ty as i32, tz as i32));
                queue.push_back((tx as usize, ty as usize, tz as usize));
            }
        }
    }

    let mut part2_n = 0;
    for (x, dy) in space.iter().enumerate() {
        for (y, dz) in dy.iter().enumerate() {
            for (z, _) in dz.iter().enumerate().filter(|(_, &v)| v == Elem::Rock) {
                for (dx, dy, dz) in fill_dirs {
                    let tx = x as i32 + dx;
                    let ty = y as i32 + dy;
                    let tz = z as i32 + dz;
                    if space[tx as usize][ty as usize][tz as usize] == Elem::Water {
                        part2_n += 1;
                    }
                }
            }
        }
    }

    println!("part2: {:?}", part2_n);
}
