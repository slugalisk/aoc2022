use std::fs;

use itertools::Itertools;

fn is_visible(grid: &[Vec<u32>], x: usize, y: usize) -> bool {
    let v = *grid.get(y).unwrap().get(x).unwrap();
    grid.get(y).unwrap().iter().take(x).all(|t| *t < v)
        || grid.get(y).unwrap().iter().skip(x + 1).all(|t| *t < v)
        || grid.iter().take(y).all(|t| *t.get(x).unwrap() < v)
        || grid.iter().skip(y + 1).all(|t| *t.get(x).unwrap() < v)
}

fn scenic_score(grid: &[Vec<u32>], x: usize, y: usize) -> usize {
    let v = *grid.get(y).unwrap().get(x).unwrap();
    let h = grid.len();
    let w = grid.get(0).unwrap().len();

    let dw = grid
        .get(y)
        .unwrap()
        .iter()
        .take(x)
        .rev()
        .find_position(|t| **t >= v)
        .map(|(t, _)| t);
    let de = grid
        .get(y)
        .unwrap()
        .iter()
        .skip(x + 1)
        .find_position(|t| **t >= v)
        .map(|(t, _)| t);
    let dn = grid
        .iter()
        .take(y)
        .rev()
        .find_position(|t| *t.get(x).unwrap() >= v)
        .map(|(t, _)| t);
    let ds = grid
        .iter()
        .skip(y + 1)
        .find_position(|t| *t.get(x).unwrap() >= v)
        .map(|(t, _)| t);

    let dw = if x > 0 && dw.is_none() {
        x
    } else if x == 0 {
        0
    } else {
        dw.unwrap() + 1
    };
    let de = if x < w - 1 && de.is_none() {
        w - x - 1
    } else if x == w - 1 {
        0
    } else {
        de.unwrap() + 1
    };
    let dn = if y > 0 && dn.is_none() {
        y
    } else if y == 0 {
        0
    } else {
        dn.unwrap() + 1
    };
    let ds = if y < h - 1 && ds.is_none() {
        h - y - 1
    } else if y == h - 1 {
        0
    } else {
        ds.unwrap() + 1
    };

    dw * de * dn * ds
}

fn main() {
    let input = fs::read_to_string("./inputs/08.txt").unwrap();

    let grid = input
        .split('\n')
        .filter(|t| !t.is_empty())
        .map(|t| {
            t.chars()
                .map(|t| t.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut part1_count = 0;
    for y in 0..grid.len() {
        for x in 0..grid.get(0).unwrap().len() {
            if is_visible(&grid, x, y) {
                part1_count += 1;
            }
        }
    }

    println!("part1 {:?}", part1_count);

    let mut part2_max = 0;
    for y in 0..grid.len() {
        for x in 0..grid.get(0).unwrap().len() {
            let score = scenic_score(&grid, x, y);
            if score > part2_max {
                part2_max = score;
            }
        }
    }

    println!("part2 {:?}", part2_max);
}
