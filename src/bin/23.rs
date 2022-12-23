use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn get_move(elves: &HashSet<(isize, isize)>, i: isize, p: (isize, isize)) -> (isize, isize) {
    let (x, y) = p;
    let test = (
        elves.contains(&(x - 1, y - 1)), // NW
        elves.contains(&(x, y - 1)),     // N
        elves.contains(&(x + 1, y - 1)), // NE
        elves.contains(&(x + 1, y)),     // E
        elves.contains(&(x + 1, y + 1)), // SE
        elves.contains(&(x, y + 1)),     // S
        elves.contains(&(x - 1, y + 1)), // SW
        elves.contains(&(x - 1, y)),     // W
    );

    if let (false, false, false, false, false, false, false, false) = test {
        return (x, y);
    }

    let mut moves = vec![(-1, x, y)];
    if let (false, false, false, _, _, _, _, _) = test {
        moves.push(((3 + i) % 4, x, y - 1))
    }
    if let (_, _, _, _, false, false, false, _) = test {
        moves.push(((2 + i) % 4, x, y + 1))
    }
    if let (false, _, _, _, _, _, false, false) = test {
        moves.push(((1 + i) % 4, x - 1, y))
    }
    if let (_, _, false, false, false, _, _, _) = test {
        moves.push(((i) % 4, x + 1, y))
    }
    moves.sort_by(|a, b| b.0.cmp(&a.0));
    (moves[0].1, moves[0].2)
}

fn get_bounds(elves: &HashSet<(isize, isize)>) -> ((isize, isize), (isize, isize)) {
    let mut xr = (isize::MAX, isize::MIN);
    let mut yr = (isize::MAX, isize::MIN);
    for (x, y) in elves {
        xr.0 = xr.0.min(*x);
        xr.1 = xr.1.max(*x);
        yr.0 = yr.0.min(*y);
        yr.1 = yr.1.max(*y);
    }
    ((xr.0, yr.0), (xr.1, yr.1))
}

fn debug_print_elves(elves: &HashSet<(isize, isize)>) {
    let (min, max) = get_bounds(elves);
    println!("{:?} {:?}", min, max);
    for y in (min.1 - 1)..=(max.1 + 1) {
        for x in (min.0 - 1)..=(max.0 + 1) {
            if elves.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn distribute_elves(elves: &HashSet<(isize, isize)>, n: isize) -> (HashSet<(isize, isize)>, isize) {
    let mut elves = elves.clone();
    for i in 0..n {
        let mut proposals: HashMap<(isize, isize), i32> = HashMap::new();
        let mut next_elves = HashSet::new();
        for &e in elves.iter() {
            let ne = get_move(&elves, i, e);
            *proposals.entry(ne).or_default() += 1;
        }
        for &e in elves.iter() {
            let ne = get_move(&elves, i, e);
            if proposals[&ne] == 1 {
                next_elves.insert(ne);
            } else {
                next_elves.insert(e);
            }
        }
        if elves == next_elves {
            return (elves, i);
        }
        elves = next_elves;
    }
    (elves, n)
}

fn main() {
    let input = fs::read_to_string("./inputs/23.txt").unwrap();
    let elves = input
        .trim()
        .split('\n')
        .enumerate()
        .collect::<Vec<_>>()
        .iter()
        .flat_map(|(y, t)| {
            t.chars()
                .collect::<Vec<_>>()
                .into_iter()
                .enumerate()
                .filter(|(_, t)| *t == '#')
                .map(|(x, _)| (x as isize, *y as isize))
        })
        .collect::<HashSet<_>>();

    let (part1_elves, _) = distribute_elves(&elves, 10);
    let (min, max) = get_bounds(&part1_elves);
    println!(
        "part1: {:?}",
        ((max.0 - min.0 + 1) * (max.1 - min.1 + 1)) - elves.len() as isize
    );

    let (_, part2_final_round) = distribute_elves(&elves, 100000);
    println!("part2: {:?}", part2_final_round + 1);
}
